use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_465165328: FileType = FileType {
    file_format: &FileFormat {
        id: 465_165_328,
        source_type: SourceType::Linguist,
        name: "JetBrains MPS",
        extensions: &["mpl", "mps", "msd"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
