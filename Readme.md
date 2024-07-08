# CCWC

`ccwc` - print newline, word, and byte counts for each file.

# Synopsis
```sh
ccwc [OPTION]... [FILE]...
```

# Description
Print newline, word and byte counts for each file and a total line if mroe than one file is specifies, with no file, or when file is -, read standard input.

# Options
```sh
-c              print the byte counts
-m              print the character counts
-l              print the newline counts
-w              print the word counts
--help          display this help and exit
```

# Usage 
TTo use `ccwc`, specify an option and a filename. If no filename is provided, it reads from the standard input.

#Examples

1. Count bytes in a file:

```sh
ccwc -c filename
```

2. Count characters in a file:

```sh
ccwc -m filename
```

3. Count lines in a file
```sh
ccwc -l filename
```

4. count words in a file:
```sh
ccwc -w filename
```

5. Count lines, words and bytes from the standard input:
```sh
echo "Sample text" | ccwc
```

## Contributing

We welcome contributioons to enhance this project. Please follow these steps:

**1.** Fork the repository.

**2.** Clone your forked copy of the project. 

```sh

git clone https://github.com/<your_name>/ccwc.git 

```

**3.** Navigate to the project directory.

```sh

cd ccwc

```

**4.** Add a reference(remote) to the original repository. 
```sh

git remote add upstream https://github.com/visheshrwl/ccwc

```

**5.** Check the remotes for this repository.
```sh

git remote -v

```

**6.** Always take a pull from the upstream repository to your master branch to keep it at par with the main project(updated repository). 
```sh

git pull upstream main

```

**7.** Create a new branch. 
```sh

git checkout -n <your_branch_name>

```

**8.** Perform your desired changes to the code base on that branch.

**9.** Track your changes. 
```sh

git add .

```

**10.** Commit your changes. 
```sh

git commit -m "Relevant message"

```

**11.** Push the committed changes in your feature branch to your remote repo. 
```sh

git push -u origin <your_branch_name>

```

**12.** To create a pull request, click on `compare and pull requests. Please ensure you compare your feature branch to the desired branch of the repository you are supposed to make a PR to.

**13.** Add an appropriate title and description to your pull request explaining your changes and efforts.

**14.** Click on `Create Pull Request.`

**15.** Voila! You have made a PR to Bug Buster's Community Website. Sit back patiently and relax while your PR is reviewed.


<h2 align="center"> Project Maintainers & Admins</h2>
<div align="center">
    <a href="https://github.com/visheshrwl">
    <img src="https://avatars.githubusercontent.com/u/92795514?v=4" width=100px height=100px />
    </a>
    <p align="center">Vishesh Rawal</p>
</div>

<h2 align="center"> Project Contributors </h2>

<div align="center">
<a href="https://github.com/visheshrwl/ccwc/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=visheshrwl/ccwc" />
</a>
</div>

