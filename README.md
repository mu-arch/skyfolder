# <img align="right" src="hosted/logo.png" alt="SkyFolder Logo" title="SkyFolder" width="250px" height="250px"> SkyFolder


### Easily share your files & folders online with minimal configuration BS. It's like your own personal Google Drive!

SkyFolder is a self-contained HTTP/Bittorrent server for sharing files with friends & colleagues. Made for busy people that want to share files NOW, and don't want to remember how to configure stuff. Ships with good defaults and includes a built-in management graphical interface.

### Screenshot:
(coming soon)

### Features:
- Downloads & uploads
- Create, Delete, Rename directories
- Sort the list by multiple parameters
- Password and ACL policy on folders
- TLS cert support with built in automatic Let's Encrypt client!
- No third party services. Runs completely on your PC or server
- Multiple views: list view, icon view, gallery view
- Low resource footprint (~10mb DRAM when idle, suitable for running 24/7 in background)
- Run on GNU/Linux, BSD, Windows, Mac
- Bittorrent tracker (incomplete, in research)
- Run for years without restart if needed
- Secure to expose to the open internet
- Management GUI that generates Gura markup for you in the background (not required, you can edit the Gura yourself)
- Multiple download methods: HTTP GET, HTTP JS managed File System Access API based download, built in bittorrent tracker that supports downloading via torrent magnet.
- Utilizes all CPU cores and available bandwidth, supports partial files and retries, generates thumbnails, supports directories with thousands of files

### Help needed:
- Mobile version
- Dark mode
- Testing
- Implementing Bittorrent
- Polish and improve UI, more clever thoughtfulness towards the UI is always desired
- Your feedback (there's no such thing as a stupid/trite question/comment here)
- Test if it works on a 32bit computer

### How to install:

#### GNU Linux (x86 & aarch64):

##### Via Cargo (recommended)

1. Install Rustlang on your computer (execute the command at https://www.rust-lang.org/learn/get-started)
2. Execute this command: `cargo install skyfolder`
3. Done. Use the command `skyfolder` to run the program

##### Via release binary

1. Download a binary, set its execution bit, execute it with `./skyfolder`
2. Done. Consider adding it to your path.

#### Apple Macintosh (x86 & aarch64): 

##### Via Cargo (recommended)

1. Install Rustlang on your computer (execute the command at https://www.rust-lang.org/learn/get-started)
2. Execute this command: `cargo install skyfolder`
3. Done. Use the command `skyfolder` to run the program

#### Microsoft Windows:

Coming soon.

## Security notice

If you plan to use the management features from outside your LAN (over the internet) it's important to use a TLS cert otherwise your management credentials can be sniffed.