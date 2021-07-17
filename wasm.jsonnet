{
  ID: "wasm",
  versions: [
    {
      name: "wasm1.0",
      version: "1.0",
      images: [{
        phase: "compilation",
        image: "becavalier/kubeless:v1@sha256:38558a2c9ed625ae3ba07af08648641b133ff5b5baa899fb28ab832faa238270",
        command: "/bin/bash /preload.sh"
       }, {
        phase: "runtime",
        image: "becavalier/kubeless:rt-v1@sha256:e9c960ada23d2c68f7f4e7f5e852a52e01f4fdba4d720945470d35b940ba38f8",
        env: {
          ROCKET_ENV: "production",
        },
      }],
    }
  ],
  depName: "",
  fileNameSuffix: ".wasm"
}
