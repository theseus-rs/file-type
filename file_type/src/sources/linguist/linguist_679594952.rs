use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_679594952: FileFormat = FileFormat {
    id: 679_594_952,
    source_type: SourceType::Linguist,
    name: "Visual Basic 6.0",
    extensions: &["Dsr", "bas", "cls", "ctl", "frm"],
    media_types: &["text/x-vb"],
    signatures: &[],
    related_formats: &[],
};
