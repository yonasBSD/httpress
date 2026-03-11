Output demo.gif

Set Shell "fish"

Set Theme "OneDark"

Set FontSize 20
Set FontFamily "FiraCode Nerd Font"

Set Width 1200
Set Height 680

Set Padding 40
Set Margin 40
Set MarginFill "#C678DD"
Set BorderRadius 10

Set TypingSpeed 100ms

Set Framerate 80

# Set fish colors and disable autosuggestions 
Hide
Type "set -g fish_autosuggestion_enabled 0"
Enter
Type "set fish_color_command 61afef --bold"
Enter
Type "set fish_color_param abb2bf"
Enter
Type "set fish_color_option e5c07b"
Enter
Type "set fish_color_quote 98c379"
Enter
Type "set fish_color_redirection c678dd"
Enter
Type "set fish_color_error e06c75 --bold"
Enter
Sleep 500ms
Show

Ctrl+L
Sleep 1s

# Scene 1: Show help
Type "httpress -h"
Sleep 1s
Enter
Sleep 8s

Ctrl+L
Sleep 500ms

# Scene 2: GET with duration, concurrency, rate, timeout
Type "httpress https://httpbin.org/get -c 20 -d 5s -r 100 -t 10"
Enter
Sleep 14s

Ctrl+L
Sleep 500ms

# Scene 3: POST with method, headers, body, fixed request count
Type `httpress https://httpbin.org/post -m post -c 5 -n 50 -H "Content-Type: application/json" -H "X-Demo: httpress" -b '{"hello":"world"}'`
Enter
Sleep 20s
