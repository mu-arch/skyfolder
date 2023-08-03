<p align="center">
  <img src="https://github.com/mu-arch/skyfolder/blob/master/hosted/logo.png" alt="Skyfolder - Securely host files onto the web directly from your PC" title="SkyFolder" width="250px" height="250px">
</p>

# Skyfolder - turn any device into a file-server accessible on the web

**It's like your own personal Google Drive!**

Skyfolder is a secure, self-contained, portable, HTTP/Bittorrent server that hosts a website for immediate file-sharing with friends, colleagues, or just yourself. Made for busy people that want to share files NOW, and don't want to remember how to configure anything. Ships with good defaults and includes a built-in management graphical interface.

No need to install anything. SkyFolder is a single portable executable file.

Skyfolder Discord: https://discord.gg/VBMe2rcYb6

### Screenshot:

![Screenshot](Screenshot%202023-08-02%20at%2010.53.24%20PM.png)

### Priorities:
1. Secure
2. Braindead simple
3. Stylish

### Features:

1. Downloads & uploads (Both pausable/resumable).
2. Create, Delete, Rename directories.
3. Sort the list by multiple parameters.
4. Access Control Lists (ACL) for managing permissions on a per folder/user basis.
5. Portable - requires no installation or management of program files; everything is packaged in a single executable file.
6. No third party services or data collection. Runs completely on your PC or server.
7. Multiple views: list view, icon view.
8. In-browser file-viewer and streamer.
9. Built in real-time search engine with fuzzy finding.
10. Low resource footprint (~10mb DRAM when idle, files are streamed off disk in small chunks) suitable to run 24/7.
11. Compatible with GNU/Linux, BSD, Windows, and Mac.
12. Supports TLS certificates, including a built-in automatic Let's Encrypt client that can renew certs with no user interaction.
13. Stable enough to run for years without needing a restart.
14. Secure to expose to the open internet.
15. Management GUI that generates [Gura](https://github.com/gura-conf/gura) markup in the background (manual Gura editing is also an option).
16. Multiple download methods: HTTP GET, HTTP JS managed File System Access API, Bittorrent magnet.
17. Efficiently utilizes all CPU cores and available bandwidth, capable of serving files at gigabit speeds to hundreds of active downloaders.
18. Supports partial files (content-range) and retries.
19. Generates thumbnails for images and videos.
20. Supports directories with tens of thousands of files.
21. Produces the correct headers for streaming video to VRChat movie theater worlds ;)
22. Supports automatic port mapping with UPnP
23. QR code generation

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

I will add this soon as I'm adding some OS specific features for each build related to GUI for starting and stopping the program.

## Security notice

1. You should use a Transport Layer Security (TLS) certificate when using management features from outside your local area network (LAN) - such as over the internet - to avoid your management credentials from being sniff-able.

2. Skyfolder has not yet been penetration tested.

3. Bittorrent Magnet links can be shared freely and used by anyone, so keep that in mind.

4. TLS certificates are integral for encrypting data in transit for all websites, including Skyfolder. However, it's important to understand the limitations of this setup. While TLS provides secure transmission, it does not equate to end-to-end (E2E) encryption. Specifically, Skyfolder does not utilize E2E encryption.

## Limitations

1. Most functionality requires Javascript to work