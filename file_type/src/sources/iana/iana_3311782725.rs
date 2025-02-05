use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3311782725: FileFormat = FileFormat {
    id: 3_311_782_725,
    source_type: SourceType::Iana,
    name: "vnd.DMClientScript",
    extensions: &[],
    media_types: &["text/vnd.DMClientScript"],
    signatures: &[],
    related_formats: &[],
};
