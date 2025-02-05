use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2346253552: FileFormat = FileFormat {
    id: 2_346_253_552,
    source_type: SourceType::Iana,
    name: "vnd.comicbook-rar",
    extensions: &[],
    media_types: &["application/vnd.comicbook-rar"],
    signatures: &[],
    related_formats: &[],
};
