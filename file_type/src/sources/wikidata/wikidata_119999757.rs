use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119999757: FileType = FileType {
    file_format: &FileFormat {
        id: 119_999_757,
        source_type: SourceType::Wikidata,
        name: "DJ RingTone File",
        extensions: &["djr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
