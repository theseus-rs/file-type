use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308768: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_768,
        source_type: SourceType::Wikidata,
        name: "DirectMusic Segment File Format",
        extensions: &["sgt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
