#[cfg(test)]
pub mod tests {
    use crate::rope::Rope;

    #[test]
    fn test_concat() {
        let h = Rope::from_str("hello");
        let w = Rope::from_str("world");

        let res = Rope::concat(h, w);
        assert_eq!(res.to_string().unwrap(), "helloworld");
    }

    #[test]
    fn test_rope_length() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        assert_eq!(rope.len(), rope.len());
    }

    #[test]
    fn test_rope_init() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        assert_eq!(rope.len(), rope.len());
    }

    #[test]
    fn test_small() {
        let s = "abc";
        let rope = Rope::from_str(s);

        let res = rope.index(0);
        assert_eq!('a', res.unwrap());

        let res = rope.index(1);
        assert_eq!('b', res.unwrap());

        let res = rope.index(2);
        assert_eq!('c', res.unwrap());
    }

    #[test]
    fn test_index() {
        let s = "123456789";
        let rope = Rope::from_str(s);
        assert_eq!('4', rope.index(3).unwrap());
    }

    #[test]
    fn test_index_none() {
        let s = "";
        let rope = Rope::from_str(s);
        let res = rope.index(1);
        assert_eq!(None, res);
    }

    #[test]
    fn test_insert_at() {
        let s = "hello world";
        let mut rope = Rope::from_str(s);
        rope.insert(5, " CTHULHU");
        assert_eq!(rope.to_string().unwrap(), "hello CTHULHU world")
    }

    #[test]
    fn test_to_string() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        assert_eq!(s, rope.to_string().unwrap());
    }

    #[test]
    fn test_split() {
        let s = "hello world";
        let rope = Rope::from_str(s);
        let (hello, rest) = rope.split(5);
        let hello = hello.to_string().unwrap();
        let world = rest.split(1).1.to_string().unwrap();

        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
    }

    #[test]
    fn test_split_big() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        let (left, right) = rope.split(74);
        assert_eq!(
            left.to_string().unwrap(),
            "hi there hello world there once was a kitten that had a chocolate ice crea",
        );
        assert_eq!(right.to_string().unwrap(), "m");
    }

    #[test]
    fn test_split_oob() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        let (left, right) = rope.split(80);

        assert_eq!(
            left.to_string().unwrap(),
            "hi there hello world there once was a kitten that had a chocolate ice cream",
        );

        assert_eq!(right.to_string().unwrap(), "",);
    }

    #[test]
    fn test_at_zero() {
        let s = "hi there hello world there once was a kitten that had a chocolate ice cream";
        let rope = Rope::from_str(s);
        let (left, right) = rope.split(0);

        assert_eq!(
            right.to_string().unwrap(),
            "hi there hello world there once was a kitten that had a chocolate ice cream",
        );
        assert_eq!(left.to_string().unwrap(), "",);
    }

    #[test]
    fn test_delete_oob() {
        let s = "hello world";
        let mut rope = Rope::from_str(s);
        rope.delete(5, 200);
        assert_eq!("hello", rope.to_string().unwrap());
    }

    #[test]
    fn test_delete_single() {
        let s = "hello";
        let mut rope = Rope::from_str(s);
        rope.delete(0, 0);
        assert_eq!("ello", rope.to_string().unwrap());
    }

    #[test]
    fn test_delete() {
        let s = "hello world";
        let mut rope = Rope::from_str(s);
        rope.delete(5, 100);
        assert_eq!("hello", rope.to_string().unwrap());

        let s = "hello world";
        let mut rope = Rope::from_str(s);
        rope.delete(5, 6);
        assert_eq!("helloorld", rope.to_string().unwrap());

        let s = "hello world";
        let mut rope = Rope::from_str(s);
        rope.delete(0, 4);
        rope.delete(1, 6);
        assert_eq!(" ", rope.to_string().unwrap());
    }

    #[test]
    fn append_to_empty_rope() {
        let mut rope = Rope::new();
        let appended_text = "Hello";
        rope.append(appended_text);

        assert_eq!(rope.to_string().unwrap(), "Hello");
    }

    #[test]
    fn append_to_non_empty_rope() {
        let mut rope = Rope::from_str("Hello");
        let appended_text = ", world!";
        rope.append(appended_text);

        assert_eq!(rope.to_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn append_empty_string() {
        let mut rope = Rope::from_str("Hello");
        rope.append("");

        assert_eq!(rope.to_string().unwrap(), "Hello");
    }
}
