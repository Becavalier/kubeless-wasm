{
  ID: "wasm",
  versions: [
    {
      name: "wasm1.0",
      version: "1.0",
      images: [{
        phase: "compilation",
        image: "becavalier/kubeless:v1@sha256:5eeb3744b8fa850c3b178eaf57479dace551e70b8be87def33156c48b5a14314",
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
