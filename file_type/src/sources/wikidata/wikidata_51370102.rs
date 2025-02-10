use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51370102: FileType = FileType {
    file_format: &FileFormat {
        id: 51_370_102,
        source_type: SourceType::Wikidata,
        name: "Microsoft Print File",
        extensions: &["prn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
