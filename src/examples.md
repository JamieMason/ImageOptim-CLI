### Optimise a directory of images

This command will optimise all image files in your Awesome project.

    imageOptim --directory ~/Sites/Awesome # [options]

### Optimise a filtered set of images

This command will optimise just the .jpg files in your Awesome project.

    find ~/Sites/Awesome -name '*.jpg' | imageOptim # [options]

### Passing additional options

The long format for enabling options is as follows;

    imageOptim --jpeg-mini --image-alpha --quit --no-color --directory path/to/images

The equivalent of the above in short format is as follows;

    imageOptim -j -a -q -d -c path/to/images

### Adding to git pre-commit hook

Adding the below to **your_project/.git/hooks/pre-commit** will run imageoptim-CLI
each time you commit new and changed files into your project. Any files which
aren't images will be ignored.

    images=$(git diff --exit-code --cached --name-only --diff-filter=ACM -- '*.png' '*.jpg')
    $(exit $?) || (echo "$images" | imageOptim && git add $images)
