use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3159702842: FileType = FileType {
    file_format: &FileFormat {
        id: 3_159_702_842,
        source_type: SourceType::Iana,
        name: "vnd.microsoft.windows.thumbnail-cache",
        extensions: &[],
        media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
        signatures: &[],
        related_formats: &[],
    },
};
