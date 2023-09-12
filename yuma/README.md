# Yuma

The runtime for the freight packaging system. Aims to provide a useful user
facing lib to installing pkgs

## Todo
- [ ] [`add`] uses traits to remove boilerplate
    - [ ] allow chaining to remove other add functions. exp:
    ```rust
    ctx.add([
        "openssh",
        "openssh-openrc".if_hostname("steambox")
    ]);
    ```
- [ ] call update ctx is dropped if not already called and warn the user
