{
  ID: "wasm",
  versions: [
    {
      name: "wasm1.0",
      version: "1.0",
      images: [{
        phase: "compilation",
        image: "becavalier/kubeless:v1@sha256:cc553fe254376e6aca8275949f1ea01217ae32ad001ec0d393a464006da9874b",
        command: "/bin/bash /preload.sh"
       }, {
        phase: "runtime",
        image: "becavalier/kubeless:rt-v1@sha256:f29e5743130e222536864017b159bbca695ee7c013fb0303c5481e494e806127",
        env: {
          ROCKET_ENV: "production",
        },
      }],
    }
  ],
  depName: "",
  fileNameSuffix: ".wasm"
}
