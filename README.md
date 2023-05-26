# RepoWrangler

RepoWrangler downloads the contents of a GitHub repo and outputs files tailored for ingestion by language models.

## Purpose

RepoWrangler is designed to make it easy for language models to ingest the content of GitHub repositories. It downloads the repo, lets you filter out content you do not want to include, and outputs the remaining content in a format ready to be used to train or finetune a model.

## Usage

To use this program, clone this repo and run:

`cargo run [repo URL] [exclusions*]`


For example:

`cargo run https://github.com/octocat/hello-world.git type:md dir:docs`


This will:


- Clone the https://github.com/octocat/hello-world.git repo

- Include all files except those with .md extension or in the docs directory

- Output the contents of the included files in output.txt, optimized for ingestion by language models

- List the included files and sizes in files.txt

- List the excluded files in excluded.txt

- Delete the local clone of the repository



## Exclusions

You can provide two types of exclusions:


- type:[extension] to exclude files by extension. For example, type:md to exclude .md files.

- dir:[name] to exclude files in a directory. For example, dir:docs to exclude the docs directory.


You can provide multiple exclusions to filter by both file extension and directory.

### Output

The program will output two files tailored for language models:


- output.txt: The contents of all included files concatenated into a single document.

- files.txt: A list of included files and their sizes. Can be used to split output.txt back into the individual files if needed.


These files will be overwritten on each run of the program.

### License

This project is licensed under the MIT License - see the LICENSE file for details.
