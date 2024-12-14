# kubediff

<details>
  <summary>Table of Contents</summary>

- [kubediff](#kubediff)
  * [Showcase](#showcase)
  * [About The Project](#about-the-project)
    + [What](#what)
    + [Why](#why)
    + [How](#how)
  * [Getting Started](#getting-started)
    + [Prerequisite](#prerequisite)
    + [Installation](#installation)
  * [Usage](#usage)
  * [Roadmap](#roadmap)
  * [Troubleshooting](#troubleshooting))

</details>

## Showcase
![showcase](https://raw.github.com/Ramilito/kubediff/main/docs/images/kubediff-showcase.gif)
_The showcase demonstrates an upgrade of agones_


Kubediff                   |  Kubectl diff
:-------------------------:|:-------------------------:
![comparison_kubediff](https://raw.github.com/Ramilito/kubediff/main/docs/images/diff_kubediff.png)  |  ![comparison_original](https://raw.github.com/Ramilito/kubediff/main/docs/images/diff_original.png)
_Notice that kubediff has removed the hated managedFields_

## About The Project

### What
Used to sanity check and understand what has changed between environments.

This cli tool written in Rust is a wrapper around kubectl diff and is supposed to diff one or multiple projects instead of
single files against any environment you want, be it docker-desktop, dev, prod.

It takes a glob pattern to one or more projects and beautifies the output so you can get an understanding on what differences there are.

### Why
Ever asked yourself, what is deployed on my cluster? Or,
* are all changes applied to the cluster? 
* What differences are there between the environments? 
* What have I forgotten to deploy? 
* Has something changed without us knowing it? 
* Forgot to add a change in git after hotfixing it in prod? Well, look no further...well a couple of lines further, I guess...


### How
We will loop over the projects files and run kubectl build, then pipe the output into kubectl diff and then process the output of that to make it pretty.

## Getting Started

### Prerequisite

* yq is needed for less clutter (can be made optional fairly easy)

### Installation
Download and extract the binary.
```
KUBEDIFF_VERSION=0.1.7 && \
KUBEDIFF_OS=x86_64-unknown-linux-gnu && \
wget "https://github.com/Ramilito/kubediff/releases/download/${KUBEDIFF_VERSION}/kubediff_${KUBEDIFF_VERSION}_${KUBEDIFF_OS}.tar.gz" && \
mkdir -p $HOME/.kube/kubediff && tar zxpf kubediff_${KUBEDIFF_VERSION}_${KUBEDIFF_OS}.tar.gz -C $HOME/.kube/kubediff
```

Add it to your PATH in .zshrc/.bashrc.
```
export PATH="$PATH:/home/$USER/.kube/kubediff"
```

## Usage

### Configuration
Se the available commands by running kubediff -h

Regular usage would be to list your projects in the config.yaml file located at the install directory


**_Few projects, will use the kustomization file located at that path:_**
```
configs:
    include:
        - "~/projectone/serviceone/k8s"
        - "~/projecttwo/servicetwo/k8s"
```

**_Many projects (monorepo), will use glob pattern to find all services:_**
```
configs:
    include:
        - "~/monorepo/Services/**/k8s"
```

**_Many environments, will suffix the variable to end of the paths in ```config.yaml```, example below will look in "~/monorepo/Services/\**/k8s/dev"_**
```
kubediff -e dev
```

**_You can also bypass the config by passing either -path or -inplace argument:_**
```
kubediff -p ~/path-to-dir-or-file
kubediff -i 
```
## Roadmap

- [ ] Remove, make optional or include dependency on yq 
- [ ] Change theme with input 

## Troubleshooting

