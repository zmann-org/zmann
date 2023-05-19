# Distraze (Proof of Concept)

Distraze is a proof-of-concept GUI (Graphical User Interface) framework written in Rust, inspired by WPF (Windows Presentation Foundation) and XAML. The goal of Distraze is to provide a modern and efficient way to build cross-platform desktop applications using Rust's safety and performance benefits.

## Features

- **Declarative UI**: Distraze allows you to define your UI using a declarative markup language inspired by XAML. This approach separates the UI design from the application logic, making it easier to manage and maintain your codebase.

- **Cross-platform**: Distraze aims to be cross-platform, allowing you to build applications that run on Windows, macOS, and Linux without significant modifications. By leveraging the power of Rust, Distraze can take full advantage of platform-specific capabilities while providing a unified API.

- **Event-driven programming**: Distraze follows an event-driven programming model, allowing you to handle user interactions and respond to events. It provides a set of event handlers that can be attached to UI elements, enabling you to create dynamic and interactive user interfaces.

- **Layout system**: Distraze includes a flexible layout system that helps you arrange and position UI elements. You can use various layout containers, such as panels, grids, or stack panels, to achieve the desired visual structure of your application.

- **Styles and themes**: Distraze supports styling and theming of UI elements. You can define custom styles to control the appearance of different controls, and themes allow you to change the overall look and feel of your application with ease.

## Examples
> **Note**
> This code is not functional, it is a proof of concept.
```rust
use distraze::prelude::*;

fn main() {
    Application::new()
        .on_execute(|| {
            let window = Window::new()
                .with_title("Hello, Distraze!")
                .hide_titlebar()
                .with_size(400, 300);

            let view = xaml("../assets/main_window.xaml").unwrap();

            view.add_child("<Button onclick={clickEvent}>Click me Button</Button>");
            
            window.add_child(view);

            window.show();
        })
        .run();
}

fn clickEvent(args: EventArgs) {
    println!("Button clicked! {}", args.content());
    // Should print "Button clicked! Click me Button"
}
```
```xml
<View content="add_here" Width="400" Height="300" WindowStyle="None">
    <Grid Background="#FF2B2B2B">
        <TextBlock Margin="10, 0" FontSize="20" Foreground="White" Text="Hello, Distraze!" />
        <Button Width="30" Height="30" Margin="5" Content="X" HorizontalAlignment="Right" VerticalAlignment="Center" />
    </Grid>
    
    <Grid Background="White">
        <slot id="add_here"/>
    </Grid>
</View>
```

## Resources
```
http://www.cmyr.net/blog/gui-framework-ingredients.html
http://www.cmyr.net/blog/rust-gui-infra.html
https://www.warp.dev/blog/why-is-building-a-ui-in-rust-so-hard
```

## License
TODO

## Acknowledgments

Distraze is built upon the efforts of numerous open-source contributors who have developed and contributed to Rust, WPF, XAML, and related projects. Their work has laid the foundation for this proof-of-concept framework.