use nexus::imgui::Ui;

pub mod options;

trait UiExtended {
    fn header<T: AsRef<str>>(&self, text: T);
    fn selected_file<L: AsRef<str>, F: Fn()>(&self, title: L, label: L, buf: &mut String, func: F);
}

impl UiExtended for Ui<'_> {
    fn header<T: AsRef<str>>(&self, text: T) {
        self.text(text);
        self.spacing();
    }

    fn selected_file<L: AsRef<str>, F: Fn()>(
        &self,
        title: L,
        label: L,
        buf: &mut String,
        on_select: F,
    ) {
        self.text(title);
        self.input_text(&label, buf)
            .hint("File location")
            .auto_select_all(true)
            .read_only(true)
            .build();
        self.same_line();
        if self.button(format!("Select##{}", label.as_ref())) {
            on_select();
        }
    }
}

fn shorten_path(path_str: String) -> String {
    let parts: Vec<&str> = path_str.split(r#"\"#).collect();
    let last_three: Vec<&str> = parts
        .iter()
        .rev()
        .take(3)
        .copied()
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();
    format!("..\\{}", last_three.join("\\"))
}
