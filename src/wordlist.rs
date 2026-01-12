use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;

/// Wordlist manager for directory and subdomain discovery
pub struct WordlistManager {
    wordlists_dir: PathBuf,
    seclists_dir: Option<PathBuf>,
}

impl WordlistManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        let prune_dir = home.join(".prune");
        let wordlists_dir = prune_dir.join("wordlists");
        
        // Create directories if they don't exist
        fs::create_dir_all(&wordlists_dir)?;
        
        // Try to find SecLists installation
        let seclists_dir = Self::find_seclists();
        
        let manager = Self { 
            wordlists_dir,
            seclists_dir,
        };
        
        // Create default wordlists if they don't exist and SecLists not found
        if manager.seclists_dir.is_none() {
            manager.ensure_default_wordlists()?;
        }
        
        Ok(manager)
    }
    
    /// Find SecLists installation on the system
    fn find_seclists() -> Option<PathBuf> {
        // Common SecLists installation paths
        let common_paths = vec![
            "/usr/share/seclists",
            "/usr/share/wordlists/seclists",
            "/opt/seclists",
            "/opt/SecLists",
        ];
        
        // Also check home directory
        if let Some(home) = dirs::home_dir() {
            let home_paths = vec![
                home.join("SecLists"),
                home.join("seclists"),
                home.join("wordlists/seclists"),
                home.join("wordlists/SecLists"),
                home.join("Tools/SecLists"),
                home.join("tools/seclists"),
            ];
            
            for path in home_paths {
                if path.exists() && path.is_dir() {
                    println!("✓ Found SecLists at: {}", path.display());
                    return Some(path);
                }
            }
        }
        
        // Check system paths
        for path_str in common_paths {
            let path = PathBuf::from(path_str);
            if path.exists() && path.is_dir() {
                println!("✓ Found SecLists at: {}", path.display());
                return Some(path);
            }
        }
        
        println!("⚠ SecLists not found. Using built-in wordlists.");
        println!("  Install SecLists: git clone https://github.com/danielmiessler/SecLists.git ~/SecLists");
        None
    }
    
    /// Load directory wordlist (prefers SecLists if available)
    pub fn load_directory_wordlist(&self) -> Result<Vec<String>> {
        // Try SecLists first
        if let Some(ref seclists) = self.seclists_dir {
            // Use common directory wordlists from SecLists
            let seclists_paths = vec![
                seclists.join("Discovery/Web-Content/common.txt"),
                seclists.join("Discovery/Web-Content/directory-list-2.3-medium.txt"),
                seclists.join("Discovery/Web-Content/raft-medium-directories.txt"),
            ];
            
            for path in seclists_paths {
                if path.exists() {
                    println!("→ Using SecLists wordlist: {}", path.file_name().unwrap().to_string_lossy());
                    return self.load_wordlist_file(&path);
                }
            }
        }
        
        // Fallback to built-in wordlist
        let path = self.wordlists_dir.join("directories.txt");
        if !path.exists() {
            self.ensure_default_wordlists()?;
        }
        self.load_wordlist_file(&path)
    }
    
    /// Load subdomain wordlist (prefers SecLists if available)
    pub fn load_subdomain_wordlist(&self) -> Result<Vec<String>> {
        // Try SecLists first
        if let Some(ref seclists) = self.seclists_dir {
            let seclists_paths = vec![
                seclists.join("Discovery/DNS/subdomains-top1million-5000.txt"),
                seclists.join("Discovery/DNS/dns-Jhaddix.txt"),
                seclists.join("Discovery/DNS/namelist.txt"),
            ];
            
            for path in seclists_paths {
                if path.exists() {
                    println!("→ Using SecLists wordlist: {}", path.file_name().unwrap().to_string_lossy());
                    return self.load_wordlist_file(&path);
                }
            }
        }
        
        // Fallback to built-in wordlist
        let path = self.wordlists_dir.join("subdomains.txt");
        if !path.exists() {
            self.ensure_default_wordlists()?;
        }
        self.load_wordlist_file(&path)
    }
    
    /// Load a wordlist file
    fn load_wordlist_file(&self, path: &PathBuf) -> Result<Vec<String>> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read wordlist: {:?}", path))?;
        
        let words: Vec<String> = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(String::from)
            .collect();
        
        Ok(words)
    }
    
    /// Ensure default wordlists exist
    fn ensure_default_wordlists(&self) -> Result<()> {
        let dir_wordlist = self.wordlists_dir.join("directories.txt");
        if !dir_wordlist.exists() {
            fs::write(dir_wordlist, DEFAULT_DIRECTORY_WORDLIST)?;
        }
        
        let sub_wordlist = self.wordlists_dir.join("subdomains.txt");
        if !sub_wordlist.exists() {
            fs::write(sub_wordlist, DEFAULT_SUBDOMAIN_WORDLIST)?;
        }
        
        Ok(())
    }
}

// Curated directory wordlist focusing on high-value targets
const DEFAULT_DIRECTORY_WORDLIST: &str = r#"# Common directories
admin
api
assets
backup
config
css
dashboard
data
db
dev
docs
downloads
files
images
img
includes
js
lib
login
logs
media
old
public
scripts
src
static
temp
test
tmp
upload
uploads
user
users
vendor
wp-admin
wp-content
wp-includes
.git
.env
.htaccess
.well-known
robots.txt
sitemap.xml
phpinfo.php
info.php
admin.php
login.php
config.php
database.php
db.php
backup.sql
dump.sql
api/v1
api/v2
api/v3
v1
v2
v3
_admin
_api
_backup
_config
_dashboard
_dev
_test
~admin
~backup
~dev
administrator
admins
app
application
applications
backend
backups
beta
blog
cache
cgi-bin
cms
console
content
core
demo
development
docs
documentation
download
env
error
errors
etc
example
export
file
fonts
forms
forum
forums
ftp
graphql
help
home
html
http
import
include
index
installation
internal
intranet
invoice
invoices
json
jsonapi
library
local
main
manage
management
manager
marketing
mobile
mysql
new
news
oauth
old_site
pages
panel
password
pdf
php
phpmyadmin
portal
private
prod
production
profile
profiles
project
projects
protected
qa
query
register
registration
reports
rest
root
sales
sample
sandbox
search
secure
security
server
service
services
session
setup
shop
site
sites
source
sql
stage
staging
stats
status
storage
store
support
system
systems
template
templates
test_site
testing
tests
theme
themes
tools
update
updates
url
utilities
utils
version
video
videos
web
webmail
webroot
website
wp
xml
xmlrpc
.backup
.bak
.config
.dev
.git/config
.git/HEAD
.old
.orig
.save
.swp
.test
"#;

// Curated subdomain wordlist focusing on common patterns
const DEFAULT_SUBDOMAIN_WORDLIST: &str = r#"# Common subdomains
www
mail
ftp
webmail
smtp
pop
ns1
ns2
ns3
ns4
webdisk
admin
administrator
blog
forum
forums
shop
store
api
dev
development
test
testing
stage
staging
prod
production
qa
uat
demo
beta
alpha
app
apps
mobile
m
static
assets
cdn
img
images
media
files
download
downloads
upload
uploads
secure
portal
vpn
remote
intranet
internal
extranet
crm
erp
cms
dashboard
panel
control
manage
management
mysql
db
database
sql
backup
ftp2
cpanel
whm
webmail2
autodiscover
autoconfig
_domainkey
_dmarc
email
mail2
pop3
imap
smtp2
mx
exchange
owa
outlook
office
o365
cloud
proxy
gateway
firewall
monitor
monitoring
log
logs
status
health
grafana
prometheus
kibana
elastic
jenkins
gitlab
github
bitbucket
jira
confluence
wiki
docs
documentation
help
support
chat
slack
teams
zoom
meet
video
stream
live
broadcast
game
games
play
social
community
member
members
user
users
account
accounts
profile
profiles
identity
auth
oauth
sso
login
signin
register
signup
api-dev
api-test
api-stage
api-prod
v1
v2
v3
rest
graphql
ws
websocket
mobile-api
partner
partners
vendor
vendors
client
clients
supplier
suppliers
b2b
b2c
wholesale
retail
shop2
checkout
cart
payment
payments
invoice
invoices
billing
subscription
subscriptions
order
orders
tracking
shipment
shipping
logistics
warehouse
inventory
catalog
product
products
search
elastic-search
solr
redis
cache
memcache
queue
rabbitmq
kafka
worker
workers
job
jobs
cron
scheduler
mailer
newsletter
campaign
campaigns
marketing
analytics
stats
metrics
report
reports
bi
datawarehouse
etl
airflow
jupyter
notebook
lab
research
data
dataset
datasets
ml
ai
model
models
training
inference
predict
recommendation
recommender
personalization
ab-test
experiment
experiments
feature
features
release
releases
version
versions
preview
sandbox
tenant
tenants
region
regions
us
eu
asia
apac
east
west
north
south
edge
node
cluster
k8s
kubernetes
docker
container
swarm
rancher
nomad
consul
vault
secrets
keys
cert
certificate
acme
letsencrypt
security
waf
ddos
shield
guard
scan
scanner
pentest
audit
compliance
siem
soc
noc
ops
devops
sre
infra
infrastructure
terraform
ansible
puppet
chef
salt
ci
cd
pipeline
build
deploy
artifact
artifacts
package
packages
registry
harbor
nexus
artifactory
repo
repository
git
svn
mercurial
code
source
review
lint
sonar
quality
coverage
performance
perf
load
stress
chaos
canary
blue
green
old
legacy
deprecated
archive
historical
backup2
dr
disaster-recovery
failover
replica
mirror
sync
rsync
s3
storage
object
block
file
share
nas
san
nfs
smb
cifs
dav
webdav
caldav
carddav
activesync
"#;
