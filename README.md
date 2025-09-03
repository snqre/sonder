# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```



            div {
                {{
                    let sprite_url = GALAXY.read().sprite_url();
                    rsx!(
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: center;
                                min-width: 128px;
                                aspect-ratio: 1 / 1;
                                background-image: url({sprite_url});
                                background-size: contain;
                                background-position-x: center;
                                background-position-y: center;
                                background-repeat: norepeat;
                            "#
                        }
                    )
                }}
            }
            for celestial_body in GALAXY.read().celestial_bodies() {{
                let name: String = celestial_body.name().to_string();
                let population: u128 = celestial_body.population().count();
                let sprite_url: Asset = celestial_body.sprite_url();
                rsx!(
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: row;
                            justify-content: center;
                            align-items: center;
                            cursor: pointer;
                            user-select: none;
                            border-width: 1px;
                            border-style: solid;
                            border-color: #37323E;
                            border-radius: 4px;
                            min-width: 200px;
                        "#,
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: center;
                                min-width: 64px;
                                aspect-ratio: 1 / 1;
                                background-image: url({sprite_url});
                                background-size: contain;
                                background-position-x: center;
                                background-position-y: center;
                                background-repeat: norepeat;
                            "#
                        }
                        div {
                            style: r#"
                                display: flex;
                                flex-direction: column;

                            "#
                        }
                    }
                )
            }}