use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_163: FileType = FileType {
    file_format: &FileFormat {
        id: 163,
        source_type: SourceType::Linguist,
        name: "INI",
        extensions: &[
            "cfg",
            "cnf",
            "container",
            "dof",
            "frm",
            "ini",
            "lektorproject",
            "mount",
            "network",
            "prefs",
            "pro",
            "properties",
            "service",
            "socket",
            "target",
            "timer",
            "url",
        ],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
