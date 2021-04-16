# distilatex

> A LaTeX summarizer, that extracts all theorems & definitions from a .tex file to quickly go over handouts

## Installation

```sh
cargo install distilatex
```

<!--

### Arch Linux

Arch users can install _distilatex_ from the AUR:

```sh
paru distilatex # or any AUR helper you use
```
--> 

## Usage

```sh
distilatex my_horribly_long_handout.tex marker-1,marker-2,... > summary.tex
```

Where the marker-_n_ are of the form:

- `begin:end`: Will include content between lines starting with `\begin` and lines starting with `\end`
- `@env`: Will include content between lines starting with `\begin{env}` and lines starting with `\end{env}`

For example, if your prof. puts theorems in a `theorem` environment and definitions between `\def` and `\enddef` commands, use `@theorem,def:enddef`.

### Opening the resulting PDF straight away

<!-- Use `--open`, it will compile the latex to a temporary file, render it as a PDF, open it using `xdg-open` and delete the PDF. -->

A `--open` flag that does just that is planned, but, in the meantime, this works:

```fish
#!/usr/bin/fish
function distilatex-quick 
  set texname (mktemp ./XXXXXXX.tex)
  set pdfname (echo $texname | string replace .tex .pdf)
  distilatex $argv[1] $argv[2] > $texname
  pdflatex -interaction=nonstopmode $texname
  xdg-open $pdfname
  for ext in aux fdb_latexmk fls log toc synctex.gz out x.gnuplot synctex\(busy\) pdf tex
    rm (echo $texname | string replace .tex .$ext)
  end
end
```

<!--

### Reading from standard input

You can use `-` as the input file name to tell _distilatex_ to read the file from standard input.

You can also use this to read _markers_ from standard input, but as a result, you cannot do both.

-->

### Typing out all of those markers every time is way too long!

You can save those markers to a file, and do the following:

```bash
distilatex input.tex $(cat markers.txt) > output.tex
```

(or if you are using fish:)
```fish
distilatex input.tex (cat markers.txt) > output.tex
```
