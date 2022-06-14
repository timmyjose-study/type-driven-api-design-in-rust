struct AccessDemo {
    user: User,
}

impl AccessDemo {
    fn render_admin_panel(&self, _admin: admin::Admin) {
        println!("Rendered admin panel");
    }

    fn render_404(&self) {
        println!("Rendered 404");
    }

    // explicit checks
    pub fn admin_panel_route_ok(&self) {
        if let Some(admin) = self.try_admin() {
            self.render_admin_panel(admin);
        } else {
            self.render_404();
        }
    }

    // no checks here
    pub fn admin_panel_route_whoops(&self) {
        // this is now a compile-time error
        //self.render_admin_panel();
    }

    fn set_user(&mut self, user: User) {
        self.user = user;
    }
}

struct User {
    name: String,
    is_admin: bool,
}

impl User {
    pub fn new(name: String, is_admin: bool) -> Self {
        User { name, is_admin }
    }
}

mod admin {
    pub struct Admin;

    impl super::AccessDemo {
        pub fn try_admin(&self) -> Option<Admin> {
            if self.user.is_admin {
                Some(Admin)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_panel() {
        let bob = User::new("Bob".to_string(), true);
        let mut access = AccessDemo { user: bob };
        access.admin_panel_route_ok();

        access.set_user(User::new("dave".to_string(), false));
        access.admin_panel_route_ok();
    }
}
