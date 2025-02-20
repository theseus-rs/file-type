use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_891511949: FileType = FileType {
    file_format: &FileFormat {
        id: 891_511_949,
        source_type: SourceType::Iana,
        name: "vnd.ecip.rlp",
        extensions: &[],
        media_types: &["application/vnd.ecip.rlp"],
        signatures: &[],
        related_formats: &[],
    },
};
