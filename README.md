# Daily Text Terminal Reader
A simple tool to read the daily text provided in [wol.jw.org](https://wol.jw.org/).

## Usage
To run Daily Text, just run the following command:
```bash
$ daily-text
Saturday, June 04

Tenho prazer em fraquezas, . . . pois, quando estou fraco, então é que sou poderoso. — 2 Cor. 12:10.

Você está acamado ou preso a uma cadeira de rodas? Sente suas pernas fracas ou está com a visão ruim? Se esse for o seu caso, será que você pode correr com os que são...
```
> To change the language, you need to open [wol.jw.org](https://wol.jw.org/) and wait until the full link is displayed, then you should copy the link and change the file on your config folder (default as ~/.config/daily-text_viewer on Linux), adding a / on the end. 
If you want just one of those elements, then you can specify it with ´--verse´ or ´--citation´. See the
example below:
```bash
$ daily-text --verse
Tenho prazer em fraquezas, . . . pois, quando estou fraco, então é que sou poderoso. — 2 Cor. 12:10.
```
This can be util if you want to feed something with these parts. More info can be displayed with `-h`
command.

### Plans
The program is complete itself, if you want to make any modification too, feel free to do it. Therefore,
I plan to add some functions later, like:
- [] Get link to open the website of today text;
- [] Split in even more parts both the verse and citation;
- [] Localize the date displayed on start;
- [] Highlighting;

