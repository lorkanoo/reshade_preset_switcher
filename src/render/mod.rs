use nexus::imgui::Ui;

pub mod options;
mod quick_access;

trait UiExtended {
    fn section<T: AsRef<str>>(&self, header: T);
    fn header<T: AsRef<str>>(&self, text: T);
    fn selected_file<L: AsRef<str>, F: Fn()>(&self, title: L, label: L, buf: &mut String, func: F);
}

impl<'ui> UiExtended for Ui<'ui> {
    fn section<T: AsRef<str>>(&self, header: T) {
        self.spacing();
        self.separator();
        self.spacing();
        self.header(header);
    }

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
