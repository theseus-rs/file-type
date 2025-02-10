use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_244: FileType = FileType {
    file_format: &FileFormat {
        id: 244,
        source_type: SourceType::Linguist,
        name: "NetLinx",
        extensions: &["axi", "axs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
