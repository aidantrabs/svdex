Output assets/demo.gif

Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 20

Type "# svdex - svd image compression lab"
Enter
Sleep 1s

Type "cargo run -- info assets/images/picture.jpg"
Enter
Sleep 5s

Type "cargo run -- compress assets/images/picture.jpg -k 20"
Enter
Sleep 5s

Type "cargo run -- compress assets/images/picture.jpg -k 100"
Enter
Sleep 5s

Type "cargo run -- experiment assets/images/picture.jpg -r 5,20,50,100"
Enter
Sleep 8s

Sleep 2s
