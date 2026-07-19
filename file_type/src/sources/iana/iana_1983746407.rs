use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1983746407: FileType = FileType {
    file_format: &FileFormat {
        id: 1_983_746_407,
        source_type: SourceType::Iana,
        name: "vnd.pasti-stx-disk-image",
        extensions: &[],
        media_types: &["application/vnd.pasti-stx-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
