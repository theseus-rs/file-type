use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128210388: FileType = FileType {
    file_format: &FileFormat {
        id: 128_210_388,
        source_type: SourceType::Wikidata,
        name: "Xcode config",
        extensions: &["xcconfig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
