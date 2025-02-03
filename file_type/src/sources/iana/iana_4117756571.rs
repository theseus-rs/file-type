use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4117756571: FileFormat = FileFormat {
    id: 4_117_756_571,
    source_type: SourceType::Iana,
    name: "vnd.gentoo.manifest",
    extensions: &[],
    media_types: &["application/vnd.gentoo.manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
