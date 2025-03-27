use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308790: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_790,
        source_type: SourceType::Wikidata,
        name: "DirectMusic Style File Format",
        extensions: &["sty"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
