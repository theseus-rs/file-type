use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2843860669: FileType = FileType {
    file_format: &FileFormat {
        id: 2_843_860_669,
        source_type: SourceType::Iana,
        name: "vnd.msa-disk-image",
        extensions: &[],
        media_types: &["application/vnd.msa-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
