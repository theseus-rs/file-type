use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_847390359: FileFormat = FileFormat {
    id: 847_390_359,
    source_type: SourceType::Iana,
    name: "vnd.easykaraoke.cdgdownload",
    extensions: &[],
    media_types: &["application/vnd.easykaraoke.cdgdownload"],
    internal_signatures: &[],
    related_formats: &[],
};
