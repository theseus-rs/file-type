use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757903: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_903,
        source_type: SourceType::Wikidata,
        name: "Gnumeric file format",
        extensions: &[],
        media_types: &["application/x-gnumeric"],
        signatures: &[],
        related_formats: &[],
    },
};
