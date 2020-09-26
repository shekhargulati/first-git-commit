# first-git-commit

A stupid Rust API to find first commit of any Git repository. It is built using warp

You can start the app using

```
$ cargo run
```

Then API server will start at http://localhost:3030. 

For exmaple, to find first Git commit of Kubernetes you can hit following URL

http://localhost:3030/first-commit?repo_url=https://github.com/kubernetes/kubernetes&repo_name=k8s
