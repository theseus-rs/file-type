use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66686421: FileType = FileType {
    file_format: &FileFormat {
        id: 66_686_421,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor 1-3 for DOS and 2 for Windows",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
