#JollyFolder

**Barebones program to quickly share a folder over the network.**
JollyFolder enables you to quickly host a HTTP server to share files. Designed for forgetful people that don't want to remember how to configure anything. Ships with sane defaults and includes a built in management GUI to quickly check the boxes on exactly what features you want.

### Project goals:
- Security, you should be able to host your files safely on the open internet
- Ultra minimal config with sane defaults -- No need to memorize crap or fiddle with CLI flags
- Management GUI that generates TOML for you in the background (not required, you can edit the TOML yourself)
- "Just works" uses all CPU cores, supports partial files and retries, generates thumbnails, supports directories with thousands of files
- Minimal code surface
