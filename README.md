# <img align="right" src="https://github.com/mu-arch/skyfolder/blob/master/hosted/logo.png" alt="SkyFolder Logo" title="SkyFolder" width="250px" height="250px"> SkyFolder

### Easily share your files & folders on the network with minimal configuration BS. It's like your own personal Google Drive!

SkyFolder is a self-contained HTTP/Bittorrent server for hosting a website to share files with friends & colleagues. Made for busy people that want to share files NOW, and don't want to remember how to configure anything. Ships with good defaults and includes a built-in management graphical interface.

### Screenshot:

![Screenshot](https://github.com/mu-arch/skyfolder/raw/master/Screenshot%202023-05-24%20at%207.36.56%20PM.webp)

### Features:
- Downloads & uploads (Both pausable and resumable)
- Create, Delete, Rename directories
- Sort the list by multiple parameters
- ACLs with granular control
- No third party services. Runs completely on your PC or server
- Multiple views: list view, icon view
- In-browser file-viewer and streamer
- Built in search engine
- Low resource footprint (~10mb DRAM when idle, suitable for running 24/7 in background)
- Run on GNU/Linux, BSD, Windows, Mac
- TLS cert support with built-in automatic Let's Encrypt client!
- Run for years without restart if needed
- No telemetry, the SkyFolder makes no outgoing connections, only serves clients connecting to it
- Secure to expose to the open internet
- Management GUI that generates Gura markup for you in the background (not required, you can edit the Gura yourself)
- Multiple download methods: HTTP GET, HTTP JS managed File System Access API, bittorrent magnet
- Utilizes all CPU cores and available bandwidth, supports partial files and retries, generates thumbnails, supports directories with thousands of files

### Roadmap
- Bittorrent tracker and seeding client so you can download files via your favorite client and seed to others
- OS toast notification when someone downloads a file from you

### Help needed:
- Mobile version
- Dark mode
- General testing
- Penetration Testing
- Automated tests
- Implementing Bittorrent
- Polish and improve UI, more clever thoughtfulness towards the UI is always desired
- Your feedback (there's no such thing as a stupid/trite question/comment here)
- Test if it works on a 32bit computer

### How to install:

Coming soon.

## Security notice

If you plan to use the management features from outside your LAN (over the internet) it's important to use a TLS cert otherwise your management credentials can be sniffed.

## Limitations
- Most functionality requires Javascript to work