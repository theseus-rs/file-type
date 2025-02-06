use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_164: FileFormat = FileFormat {
    id: 164,
    source_type: SourceType::Linguist,
    name: "IRC log",
    extensions: &["irclog", "weechatlog"],
    media_types: &["text/mirc"],
    signatures: &[],
    related_formats: &[],
};
