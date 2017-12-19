# rust-vn
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
