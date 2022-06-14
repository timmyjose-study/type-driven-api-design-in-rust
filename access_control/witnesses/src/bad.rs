struct AccessDemo {
    user: User,
}

impl AccessDemo {
    fn render_admin_panel(&self) {
        println!("Rendered admin panel");
    }

    fn render_404(&self) {
        println!("Rendered 404");
    }

    // explicit checks
    pub fn admin_panel_route_ok(&self) {
        if self.user.is_admin {
            self.render_admin_panel();
        } else {
            self.render_404();
        }
    }

    // no checks here
    pub fn admin_panel_route_whoops(&self) {
        self.render_admin_panel();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_panel() {
        let bob = User::new("Bob".to_string(), true);
        let mut access = AccessDemo { user: bob };
        access.admin_panel_route_ok();
        access.admin_panel_route_whoops();

        access.set_user(User::new("dave".to_string(), false));
        access.admin_panel_route_ok();
        access.admin_panel_route_whoops();
    }
}
