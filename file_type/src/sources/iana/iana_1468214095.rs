use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1468214095: FileFormat = FileFormat {
    id: 1_468_214_095,
    source_type: SourceType::Iana,
    name: "vnd.gentoo.pkgmetadata+xml",
    extensions: &[],
    media_types: &["application/vnd.gentoo.pkgmetadata+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
