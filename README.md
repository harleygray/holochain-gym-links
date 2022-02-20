# Linking holochain entries
Solution to the [Working with links](https://holochain-gym.github.io/developers/basic/links/) exercises on the Holochain Gym. 

The `create_post` zome function takes user input to make an entry. A link is automatically made between the new entry and the user's public key, with the tag "author". 

The `get_posts_for_agent` zome function accepts a public key and retrieves all entries made by the corresponding user.


### Setup - nix-shell
IMPORTANT: These need to be run in the correct nix-shell. 

In the base folder of this repository, developer-exercises, you will find
a `default.nix` file. Run the following command in your terminal:

```bash
nix-shell
```

The very first time you run this, it will take long time, somewhere between 20 and 80 minutes.
This is because it will download, install and compile everything you need. After that it will only take a second or two to run.

