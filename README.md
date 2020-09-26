# first-git-commit

A stupid Rust API to find first commit of any Git repository. It is built using warp

You can start the app using

```
$ cargo run
```

Then API server will start at http://localhost:3030. 

For exmaple, to find first Git commit of Kubernetes you can hit following URL

http://localhost:3030/first-commit?repo_url=https://github.com/kubernetes/kubernetes&repo_name=k8s

The ouput will be

```
commit 2c4b3a562ce34cddc3f8218a2c4d11c7310e6d56
Author: Joe Beda <joe.github@bedafamily.com>
Date:   Fri Jun 6 16:40:48 2014 -0700
```
