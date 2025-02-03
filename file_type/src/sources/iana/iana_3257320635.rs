use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3257320635: FileFormat = FileFormat {
    id: 3_257_320_635,
    source_type: SourceType::Iana,
    name: "vnd.debian.binary-package",
    extensions: &[],
    media_types: &["application/vnd.debian.binary-package"],
    internal_signatures: &[],
    related_formats: &[],
};
