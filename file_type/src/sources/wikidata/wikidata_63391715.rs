use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63391715: FileType = FileType {
    file_format: &FileFormat {
        id: 63_391_715,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor Macintosh, version 4",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
