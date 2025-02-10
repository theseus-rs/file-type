use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120984438: FileType = FileType {
    file_format: &FileFormat {
        id: 120_984_438,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher Template",
        extensions: &["pub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
