use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4274586742: FileFormat = FileFormat {
    id: 4_274_586_742,
    source_type: SourceType::Iana,
    name: "vnd.nokia.conml+wbxml",
    extensions: &[],
    media_types: &["application/vnd.nokia.conml+wbxml"],
    signatures: &[],
    related_formats: &[],
};
