# <img align="right" src="hosted/logo.png" alt="SkyFolder Logo" title="SkyFolder" width="250px" height="250px"> SkyFolder


**Easily share your files & folders online with minimal configuration BS. It's like your own personal Google Drive!**

SkyFolder is a self-contained HTTP/Bittorrent server for sharing files with friends & colleagues. Made for busy people that want to share files NOW, and don't want to remember how to configure stuff. Ships with good defaults and includes a built-in management graphical interface.

### Project goals:
- Run on GNU/Linux, BSD, Windows, Mac
- Security, safety when exposed to the open internet permanently
- Minimal config with good defaults -- No need to memorize crap or fiddle with CLI flags
- Management GUI that generates Gura markup for you in the background (not required, you can edit the TOML yourself)
- "Just works" -- utilizes all CPU cores and available bandwidth, supports partial files and retries, generates thumbnails, supports directories with thousands of files
- Multiple download methods: HTTP GET, HTTP JS managed File System Access API based download, built in bittorrent tracker that supports downloading via torrent magnet.

### Features:
- Downloads
- Uploads
- List directories
- Password and ACL policy on folders
- Bittorrent tracker (incomplete, in research)

### Help needed:
- Mobile version
- Testing
- Implementing Bittorrent
- Polish and improve UI, more clever thoughtfulness towards the UI is always desired
- Your feedback (there's no such thing as a stupid/trite question/comment here)
