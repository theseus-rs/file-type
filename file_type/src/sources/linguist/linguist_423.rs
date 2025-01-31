use crate::format::FileFormat;

pub(crate) const LINGUIST_423: FileFormat = FileFormat {
    id: 423,
    puid: "linguist/423",
    name: "JSON with Comments",
    extensions: &[
        "code-snippets",
        "code-workspace",
        "jsonc",
        "sublime-build",
        "sublime-color-scheme",
        "sublime-commands",
        "sublime-completions",
        "sublime-keymap",
        "sublime-macro",
        "sublime-menu",
        "sublime-mousemap",
        "sublime-project",
        "sublime-settings",
        "sublime-theme",
        "sublime-workspace",
        "sublime_metrics",
        "sublime_session",
    ],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
