#[cfg(test)]
mod tests {

    #[test]
    fn test_get_user_layout_custom_env_var() {
        std::env::set_var("LAYOUTS", "ca,us,pl");
        assert_eq!(kb::get_user_layout(), &["ca", "us", "pl"]);
    }

    #[test]
    fn test_get_user_layout_empty() {
        std::env::set_var("LAYOUTS", "");
        assert_eq!(kb::get_user_layout().len(), 0);
    }

    #[test]
    fn test_get_user_layout_default_case() {
        std::env::remove_var("LAYOUTS");
        assert_eq!(kb::get_user_layout(), &["us", "fr"]);
    }

    #[test]
    fn test_get_user_layout_wrong_env_var() {
        std::env::set_var("LAYOUTS", ",");
        assert_eq!(kb::get_user_layout().len(), 0);
    }

    #[test]
    fn test_get_user_layout_duplicate() {
        std::env::set_var("LAYOUTS", "us,us");
        assert_eq!(kb::get_user_layout(), &["us"]);
    }

    #[test]
    fn test_next_layout_empty() {
        std::env::set_var("LAYOUTS", "");
        assert!(
            kb::next_layout(true).is_err(),
            "Empty variable must lead to Error"
        );
    }

    #[test]
    fn test_next_layout_current_layout_not_in_available() {
        std::env::set_var("LAYOUTS", "doesnotexist");
        let layout = kb::get_layout().unwrap();
        kb::set_layout(&layout, true).unwrap();
        assert!(
            kb::next_layout(true).is_err(),
            "Current layout not in available must lead to Error"
        );
    }
}
