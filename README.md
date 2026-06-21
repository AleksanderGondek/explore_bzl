explore_bzl
---

This CLI tool helps to onboard developers to Bazel through exploration of code repository they know well. It is a simple browser of Bazel packages and targets, which presents in approachable manner information about the configured targets and actions that will be run by the build, thus teaching by example.

![Demo preview](demo.gif)

## Table of contents

1. [Overview](#overview)
2. [How to build](#how-to-build)
2. [Roadmap](#roadmap)

## Overview

The goal of this tool is to ease the process of onboarding developers to Bazel build system. Instead of going the route of courses, documentation etc. the tool
places the power of exploration into the developers hands, thus enabling them to learn the inner workings of build system of their codebase by simply "toying" with it.

This greatly reduces the cognitive overhead required for people to learn Bazel and start being productive. 

## How to build

> [!NOTE]
> The pre-built binary will be available alongside releases made in future.

```shell
$ git clone git@github.com:AleksanderGondek/explore_bzl.git
$ bazel build //:explore_bzl
$ $(bazel info bazel-bin)/explore_bzl
```

## Roadmap
No sleep til Bazelcon!
 
* [ ] Finish Bazelification (ETA: 2026-06-30)
  * [ ] Integrate clippy
  * [ ] Integrate docs generation
  * [ ] Rearrange structure to improve composibility
* [ ] Release process & its automation (ETA: 2026-07-12)
  * [ ] Automated releases, based on conventional commits
  * [ ] Automated publication to github 'releases'
  * [ ] Automated publication to BCR
* [ ] Features
  * [ ] Visualization of aquery results (ETA: 2026-06-30)
  * [ ] Build command capability (ETA: 2026-07-05)
  * [ ] Scrolling of panes (ETA: 2026-07-26)
  * [ ] Panes selection (ETA: 2026-07-26)

