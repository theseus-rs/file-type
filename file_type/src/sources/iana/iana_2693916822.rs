use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2693916822: FileType = FileType {
    file_format: &FileFormat {
        id: 2_693_916_822,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.eclass",
        extensions: &[],
        media_types: &["application/vnd.gentoo.eclass"],
        signatures: &[],
        related_formats: &[],
    },
};
