#[test]

fn iterator_test() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // 这里的讲述不要参考书本上的，用java中的那个更合适
    // 调用 next 方法的方法被称为 消费适配器（consuming adaptors）
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), None);

    let tally: i32 = v1_iter.sum();
    assert_eq!(tally, 0);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 2]);

    // 使用闭包获取环境
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let my_size_shoe = shoes_in_my_size(shoes, 10);
        assert_ne!(
            my_size_shoe,
            vec![Shoe {
                size: 13,
                style: String::from("sandal"),
            },]
        )
    }
}
