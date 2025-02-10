use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63344874: FileType = FileType {
    file_format: &FileFormat {
        id: 63_344_874,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor 5-6",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
