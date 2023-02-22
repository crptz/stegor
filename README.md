# About this project

This project aims to create a minimal but efficient (and flexible) steganography tool that hides data inside an image using different methods defined by the user from a cli. Must feature for this is project is to encrypt the message before embedding it in image.

## Methods
Right now the only method that had been implemented and is the default method is by storing the length of the message on the first pixel in the Red channel, then it continues to the next pixels one-by-one adding in each pixel's red channel the message's data.


# Commands

### Embed data
```
stegor embed -f .\imposter.png -m "hiddenmessage"
```

**by default the image will be outputed to a new image in the same directory called** `output.png`

### Extract data
```
stegor extract -f .\output.png
```


# Todo

To be updated ...