use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_164: FileType = FileType {
    file_format: &FileFormat {
        id: 164,
        source_type: SourceType::Linguist,
        name: "IRC log",
        extensions: &["irclog", "weechatlog"],
        media_types: &["text/mirc"],
        signatures: &[],
        related_formats: &[],
    },
};
