use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3910194067: FileFormat = FileFormat {
    id: 3_910_194_067,
    source_type: SourceType::Iana,
    name: "vnd.fastcopy-disk-image",
    extensions: &[],
    media_types: &["application/vnd.fastcopy-disk-image"],
    signatures: &[],
    related_formats: &[],
};
