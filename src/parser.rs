use std::str::from_utf8;

pub enum ParseResult<T> {
    Complete(T),
    Partial,
    // 簡単のため今回はエラーデータを省略
    Error,
}

impl<T> ParseResult<T> {
    fn is_complete(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Complete(_) => true,
            _ => false
        }
    }

    fn is_partial(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Partial => true,
            _ => false
        }
    }
}

impl<T, E> From<Result<T, E>> for ParseResult<T> {
    fn from(r: Result<T, E>) -> Self {
        use self::ParseResult::*;
        match r {
            Ok(t) => Complete(t),
            Err(_) => Error,
        }
    }
}

pub struct Request<'a>(pub &'a str);

pub fn parse(mut buf: &[u8]) -> ParseResult<Request> {
    use self::ParseResult::*;

    // b".." は、バイト列リテラル
    let get = b"GET ";
    let end = b"\r\n";
    // GET がこなければエラー
    if !buf.starts_with(get) {
        return Error;
    }

    // GET をパースした残りは、パスネームと\r\n
    buf = &buf[get.len()..];
    if buf.ends_with(end) {
        buf = &buf[0..buf.len() - end.len()]
    } else {
        // 末尾が\r\nでなければ、入力が完了していないとみなす
        // 本当は途中に\r\nがある可能性もあるが、簡単のためスルー
        return Partial;
    }

    // from_utf8で、&[u8]から&strが作れる。データのコピーはしない
    // ただし失敗するかもしれないので、返り値はResultに包まれる
    from_utf8(buf)
        // タプル構造体は関数としても扱える
        // Result<&str, Utf8Error> -> Result<Request, Utf8Error>
        .map(Request)
        // Fromを実装したのでIntoのintoメソッドが自動で実装されている
        // Result<Request, Utf8Error> -> ParseResult<Request>
        .into()
}
