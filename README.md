# Rust by example
## Mục đích
Tài liệu này viết nhằm mục đích hướng dẫn từ cơ bản đến nâng cao cho người mới bắt đầu với rust.

## Rust cho người mới bắt đầu
### Cài đặt rust
* Mở terminal trên mac hoặc linux và chạy lệnh sau:

```
$ curl https://sh.rustup.rs -sSf | sh
``` 
Sau khi cài đặt rust xong chúng ta hãy thực hiện link compiler của rust bằng câu lệnh dưới
```
$ source $HOME/.cargo/env
```

Ngoài ra bạn cũng có thể sử dụng cách sau

```
$ export PATH="$HOME/.cargo/bin:$PATH"
```




* Bước tiếp theo chúng ta kiểm tra cài đặt vừa thực hiện

Mở text editor và gõ đoạn mã lệnh sau:

```
// main.rs
fn main() {
    println!("Hello Rust's World");
}
```
Mở terminal và thực hiện compile và chạy file vừa tạo nói trên


```
rustc main.rs
./main
```

Kết quả hiện ra trên terminal sẽ là: ``Hello Rust's World``

Đến đây chúng ta đã kết thúc bài học đầu tiên về rust.
### Comment
* Comment code:
	* Sử dụng ```//``` cho comment line
	* Sử dụng ```/* */``` cho comment block
* Tạo document
	* Sử dụng ```///``` cho việc tạo document với các item phía sau
	* Sử dụng ```//!``` cho việc tạo document với các item phía trên 

Cụ thể ta có thể xem ví dụ dưới đây:

```
fn main() {
    // This is an example of a line comment
    // Notice how there are two slashes at the beginning of the line
    // And that nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /* 
     * This is another type of comment, the block comment. In general,
     * the line comment is the recommended comment style however the
     * block comment is extremely useful for temporarily disabling
     * a large chunk of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out all the lines
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

     /*
     Note, the previous column of `*` was entirely for style. There's
     no actual need for it.
     */

     // Observe how block comments allow easy expression manipulation
     // which line comments do not. Deleting the comment delimiters
     // will change the result:
     let x = 5 + /* 90 + */ 5;
     println!("Is `x` 10 or 100? x = {}", x);
}
```