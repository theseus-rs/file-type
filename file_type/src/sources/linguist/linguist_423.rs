use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_423: FileType = FileType {
    file_format: &FileFormat {
        id: 423,
        source_type: SourceType::Linguist,
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
        signatures: &[],
        related_formats: &[],
    },
};
