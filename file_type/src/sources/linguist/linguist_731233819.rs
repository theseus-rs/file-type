use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_731233819: FileType = FileType {
    file_format: &FileFormat {
        id: 731_233_819,
        source_type: SourceType::Linguist,
        name: "NWScript",
        extensions: &["nss"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
