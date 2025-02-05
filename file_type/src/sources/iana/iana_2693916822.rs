use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2693916822: FileFormat = FileFormat {
    id: 2_693_916_822,
    source_type: SourceType::Iana,
    name: "vnd.gentoo.eclass",
    extensions: &[],
    media_types: &["application/vnd.gentoo.eclass"],
    signatures: &[],
    related_formats: &[],
};
