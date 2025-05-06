use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_674379998: FileType = FileType {
    file_format: &FileFormat {
        id: 674_379_998,
        source_type: SourceType::Linguist,
        name: "HIP",
        extensions: &["hip"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
