use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_528362505: FileType = FileType {
    file_format: &FileFormat {
        id: 528_362_505,
        source_type: SourceType::Iana,
        name: "vnd.evolv.ecig.settings",
        extensions: &[],
        media_types: &["application/vnd.evolv.ecig.settings"],
        signatures: &[],
        related_formats: &[],
    },
};
