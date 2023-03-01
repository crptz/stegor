# About this project
The goal of this project is to develop a steganography tool that is minimal, yet efficient and flexible, enabling users to conceal data within images using customizable methods through a command-line interface (CLI). One of the essential features of this project is the ability to encrypt the message before embedding it in the image.

# Methods
The currently implemented method for embedding a message into an image involves storing the length of the message in the Red channel of the first pixel, followed by sequentially adding the message data to each pixel's Red channel.
***This also means that currently the program supports only lossless image formats.***

# Commands

### Embed data
```
stegor embed -i ./imposter.png -m "hiddenmessage"
```
**by default the image will be outputed to a new image in the same directory called** `output.png`

You can also specify the output filename image:
```
stegor embed -i ./imposter.png -m "AGAIN AND AGAIN AND AGAIN" -o ~/path/to/other_name.png
```

### Extract data
```
stegor extract -i ./output.png
```

# Some things to note about this project
- This project depends entirely (at least for now) on [image](https://crates.io/crates/image)  crate, this means that it only supports formats that the image crate supports
- For now the project supports only one method to embed message into images
- It only supports Lossless images (PNG, BMP, GIF, TIFF)
- Also please note that GIF embeding isn't done for all frames, it takes only one frame and it outputs it, this means that it outputs an image and not a GIF.

For more please see the [TODO](#todo-list) list

# Todo List

- [✓] Support PNG images or any other Lossless format images (BMP, GIF, TIFF)
- [✗] Write the user manual
- [✗] Add support for JPEG images
- [✗] Improve performance of the encoding algorithm
- [✗] Create algorithm to store data in image instead adding them at start of the image
