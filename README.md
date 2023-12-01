# Shuttle 2023 Christmas Code Hunt

This repo is the home for my completed solutions for the 2023 Christmas Code Hunt, hosted by [Shuttle](https://shuttle.rs).

*It will be public until the morning of December 1st, 2023.*

## 2023-11-27
- completed Challenge -1 core and bonus tasks

## 2023-11-28
- implemented internal test module for the Challenge -1 core and bonus tasks
- unable to figure out how to spin up an instance of my Tower router service, without first starting the main binary with `cargo shuttle run`

## 2023-11-29
- re-factored test module using `tower-test`

## 2023-11-30
- re-factoed into a workspace format, with individual crates for each challenge
- set up workspace configuration for the Challenge; binary will continue to live in the minus1 crate, individual challenge endpoints will be in a separate crate for each day of the challenge