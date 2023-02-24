# About this project
The goal of this project is to develop a steganography tool that is minimal, yet efficient and flexible, enabling users to conceal data within images using customizable methods through a command-line interface (CLI). One of the essential features of this project is the ability to encrypt the message before embedding it in the image.

# Methods
The currently implemented method for embedding a message into an image involves storing the length of the message in the Red channel of the first pixel, followed by sequentially adding the message data to each pixel's Red channel.


# Commands

### Embed data
```
stegor embed -f ./imposter.png -m "hiddenmessage"
```

**by default the image will be outputed to a new image in the same directory called** `output.png`

### Extract data
```
stegor extract -f ./output.png
```


# Todo

To be updated ...