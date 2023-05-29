# <img align="right" src="https://github.com/mu-arch/skyfolder/blob/master/hosted/logo.png" alt="SkyFolder Logo" title="SkyFolder" width="250px" height="250px"> Skyfolder

### Easily share your files & folders on the network. Security, Simplicity, Style all built in.
### It's like your own personal Google Drive!

Skyfolder is a self-contained portable HTTP/Bittorrent server that hosts a website for immediate filesharing with friends & colleagues. Made for busy people that want to share files NOW, and don't want to remember how to configure anything. Ships with good defaults and includes a built-in management graphical interface.

No need to install anything. SkyFolder is a single portable executable file.

Skyfolder Discord: https://discord.gg/VBMe2rcYb6

### Screenshot:

![Screenshot](https://github.com/mu-arch/skyfolder/raw/master/Screenshot%202023-05-24%20at%207.36.56%20PM.webp)

### Features:
- Downloads & uploads (Both pausable and resumable)
- Create, Delete, Rename directories
- Sort the list by multiple parameters
- Access Control Lists (ACL) for managing permissions on a per folder/user basis
- Portable. Nothing to install or program files to manage. It's all conveniently packaged in a single executable file
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
- Utilizes all CPU cores and available bandwidth, can serve files at gigabit speeds with hundreds of active downloaders
- Supports partial files and retries
- Generates image and video thumbnails
- Supports directories with tens of thousands of files

### Roadmap
- Bittorrent tracker and seeding client so you can download files via your favorite client and seed to others
- OS toast notification when someone downloads a file from you

### Help needed:
- Mobile version
- Dark mode
- General testing
- Penetration Testing
- Unit tests
- Implementing Bittorrent
- Polish and improve UI, more clever thoughtfulness towards the UI is always desired
- Your feedback (there's no such thing as a stupid/trite question/comment here)
- Test if it works on a 32bit computer

### How to install:

Coming next week.

## Security notice

1. You must use a Transport Layer Security (TLS) certificate when using management features from outside your local area network (LAN) - such as over the internet - to avoid your management credentials from being sniff-able.

2. Skyfolder has not yet been penetration tested.

3. In environments that require high security Bittorrent features should be disabled. Magnet links can be shared freely and used by anyone, among numerous other concerns. Security is not part of Bittorrent's design in general.

4. TLS certificates are integral for encrypting data in transit for all websites, including Skyfolder. However, it's important to understand the limitations of this setup. While TLS provides secure transmission, it does not equate to end-to-end (E2E) encryption. Specifically, Skyfolder does not utilize E2E encryption.

5. Non-scientific anecdote, but I personally don't trust the security of Windows or Mac. If you're running a server long-term I would suggest using a Linux or BSD based OS.

## Limitations

1. Most functionality requires Javascript to work
