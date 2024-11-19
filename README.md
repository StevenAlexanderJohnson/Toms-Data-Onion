# Overview

This is my attempt at solving Tom's Data Onion using Rust.
The full project and website can be found [here](https://www.tomdalling.com/toms-data-onion).

I will have a new library to solve each layer. Below is the landing page of TDO. I have summarized it and took the input out for brevity sake.

         $$$$$$$$\                    $$\
         \__$$  __|                   $  |
            $$ | $$$$$$\  $$$$$$\$$$$\\_/$$$$$$$\
            $$ |$$  __$$\ $$  _$$  _$$\ $$  _____|
            $$ |$$ /  $$ |$$ / $$ / $$ |\$$$$$$\
            $$ |$$ |  $$ |$$ | $$ | $$ | \____$$\
            $$ |\$$$$$$  |$$ | $$ | $$ |$$$$$$$  |
            \__| \______/ \__| \__| \__|\_______/

           $$$$$$$\             $$\
           $$ |  $$ | $$$$$$\ $$$$$$\    $$$$$$\
           $$ |  $$ | \____$$\\_$$  _|   \____$$\
           $$ |  $$ | $$$$$$$ | $$ |     $$$$$$$ |
           $$ |  $$ |$$  __$$ | $$ |$$\ $$  __$$ |
           $$$$$$$  |\$$$$$$$ | \$$$$  |\$$$$$$$ |
           \_______/  \_______|  \____/  \_______|

         $$$$$$\            $$\
        $$  __$$\           \__|
        $$ /  $$ |$$$$$$$\  $$\  $$$$$$\  $$$$$$$\
        $$ |  $$ |$$  __$$\ $$ |$$  __$$\ $$  __$$\
        $$ |  $$ |$$ |  $$ |$$ |$$ /  $$ |$$ |  $$ |
        $$ |  $$ |$$ |  $$ |$$ |$$ |  $$ |$$ |  $$ |
         $$$$$$  |$$ |  $$ |$$ |\$$$$$$  |$$ |  $$ |
         \______/ \__|  \__|\__| \______/ \__|  \__|




                          _...-----.._    |/ /
                       ../////////////\__/////
                     .////          /////////#
                   .///   //////////////////#
                  ////  ////////     //////##
                 .|// //////     ///// ///##
                 |///////    ///////  ////##
                 |//////  ////////  //// /##
                 \///// ////////  ///// //##
               \~>%///////////  /////  //##
               __>%%///////   //////  //##
                ~^>%%////////////   ///##
                ~/^>%%/////////  ///###
                  ~/^%%///////////###
                    |   ##########



        >>> A PROGRAMMING PUZZLE IN A TEXT FILE <<<
                       Version 1.1.3
                      by @tom_dalling
          https://tomdalling.com/toms-data-onion/


==[ Introduction ]==========================================

At the bottom of this file you will find a payload -- a blob
of data that has been obfuscated in some way. When it is
decoded correctly, the payload will turn into another text
file with another puzzle. There are many puzzles wrapped
inside each other, like a matryoshka doll, or the layers of
an onion.

You will need to write code to do the decoding. This can be
done using any programming language.

Every layer clearly explains how to decode its payload.
These are puzzles with deterministic solutions, like Sudoku,
not riddles. I'm a software developer, not the sphinx of
Thebes.

There is a little bit of educational value in each layer. In
order to progress, you will need to learn and use computery
concepts like bitwise operations, encodings, cryptography,
error detection, and so on.


==[ Warning ]===============================================

After peeling away all the layers, you will discover...


    THECO  R  E  THE       COR    ETH   ECO   RETH
      E    C  O  R        E   T  H   E  C  O  R
      E    T  H  E        C      O   R  E  T  H
      E    CORE  THE      C      O   R  ETH   ECOR
      E    T  H  E        C   O  R   E  T  H  E
      C    O  R  ETH       ECO    RET   H  E  CORE


I've hidden something at the core of this puzzle --
something that I probably should not have published --
something that, if you read it, you might wish that you
hadn't. This may land me in serious trouble, but I can't
sleep easy at night holding on to this secret. You have been
warned.

==[ Layer 0/6: ASCII85 ]====================================

ASCII85 is a binary-to-text encoding. These encodings are
useful when you need to send arbitrary binary data as text,
such as sending an image as an email attachment, or
embedding obfuscated data in a text file. It takes four
bytes of binary data, and converts them into five printable
ASCII characters. The encoding only uses 85 "safe" ASCII
characters, hence its name.

    ----------------------------------------------------

This payload has been encoded with Adobe-flavoured ASCII85.
All subsequent layers are ASCII85 encoded just like this
one, but they require additional processing in order to be
solved.

Decode the payload below to proceed!

> This is where the input was provided.