# <img align="right" src="static/jolly_logo_long.png" alt="JollyFolder Logo" title="JollyFolder" width="200px" height="200px"> JollyFolder


**A Barebones program to quickly share folders on the network.**

JollyFolder enables you to quickly and easily spin up a self-contained HTTP server to share files. Designed for busy people that don't want to remember how to configure stuff. Ships with good defaults and includes a built-in management GUI to quickly check the boxes on exactly what features you want.

### Project goals:
- Security, should be safe to expose to the open internet
- Minimal config with good defaults -- No need to memorize crap or fiddle with CLI flags
- Management GUI that generates TOML for you in the background (not required, you can edit the TOML yourself)
- "Just works" -- utilizes all CPU cores and available bandwidth, supports partial files and retries, generates thumbnails, supports directories with thousands of files
- Multiple download methods: HTTP GET, HTTP JS managed File System Access API based download, built in bittorrent tracker that supports downloading via torrent magnet.

### Features:


### Help needed:
- Mobile version
- Testing
- Implementing bittorrent
