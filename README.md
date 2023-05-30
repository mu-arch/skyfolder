# <img align="right" src="https://github.com/mu-arch/skyfolder/blob/master/hosted/logo.png" alt="SkyFolder Logo" title="SkyFolder" width="250px" height="250px"> Skyfolder

### Easily share your files & folders on the network. Security, Simplicity, Style all built in.
### It's like your own personal Google Drive!

Skyfolder is a self-contained portable HTTP/Bittorrent server that hosts a website for immediate file-sharing with friends & colleagues. Made for busy people that want to share files NOW, and don't want to remember how to configure anything. Ships with good defaults and includes a built-in management graphical interface.

No need to install anything. SkyFolder is a single portable executable file.

Skyfolder Discord: https://discord.gg/VBMe2rcYb6

### Screenshot:

![Screenshot](https://github.com/mu-arch/skyfolder/raw/master/Screenshot%202023-05-24%20at%207.36.56%20PM.webp)

### Features:

1. Downloads & uploads (Both pausable/resumable using [TUS](https://github.com/tus/tus-resumable-upload-protocol)).
2. Create, Delete, Rename directories.
3. Sort the list by multiple parameters.
4. Access Control Lists (ACL) for managing permissions on a per folder/user basis.
5. Portable - requires no installation or management of program files; everything is packaged in a single executable file.
6. No third party services. Runs completely on your PC or server.
7. Multiple views: list view, icon view.
8. In-browser file-viewer and streamer.
9. Search folders on the client-side.
10. Low resource footprint (~10mb DRAM when idle, suitable for running 24/7 in background).
11. Compatible with GNU/Linux, BSD, Windows, and Mac.
12. Supports TLS certificates, including a built-in automatic Let's Encrypt client that can renew certs with no user interaction.
13. Stable enough to run for years without needing a restart.
14. No telemetry - Skyfolder doesn't initiate outgoing connections.
15. Secure to expose to the open internet.
16. Management GUI that generates [Gura](https://github.com/gura-conf/gura) markup in the background (manual Gura editing is also an option).
17. Multiple download methods: HTTP GET, HTTP JS managed File System Access API, Bittorrent magnet.
18. Efficiently utilizes all CPU cores and available bandwidth, capable of serving files at gigabit speeds to hundreds of active downloaders.
19. Supports partial files (content-range) and retries.
20. Generates thumbnails for images and videos.
21. Supports directories with tens of thousands of files.

### Roadmap

1. Bittorrent tracker and seeding client so you can download files via your favorite client and seed to others
2. OS toast notification when someone downloads a file from you

### Help needed:

1. Mobile version
2. Dark mode
3. General testing
4. Penetration Testing
5. Unit tests
6. Implementing Bittorrent
7. Polish and improve UI, more clever thoughtfulness towards the UI is always desired
8. Your feedback (there's no such thing as a stupid/trite question/comment here)
9. Test if it works on a 32bit computer

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
