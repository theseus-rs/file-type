use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_369: FileType = FileType {
    file_format: &FileFormat {
        id: 369,
        source_type: SourceType::Linguist,
        name: "TeX",
        extensions: &[
            "aux", "bbx", "cbx", "cls", "dtx", "ins", "lbx", "ltx", "mkii", "mkiv", "mkvi", "sty",
            "tex", "toc",
        ],
        media_types: &["text/x-stex"],
        signatures: &[],
        related_formats: &[],
    },
};
