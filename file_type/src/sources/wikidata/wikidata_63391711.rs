use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63391711: FileType = FileType {
    file_format: &FileFormat {
        id: 63_391_711,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor Macintosh, version 3",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
