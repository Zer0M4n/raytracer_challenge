mod point {
    struct Point{
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[cfg(test)]
    mod  tests{
        fn can_be_created()
        {
            let point = Point{
                x:4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0l,
            };
            assert!(true);
        }
    }

}