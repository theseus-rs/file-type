use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120635955: FileType = FileType {
    file_format: &FileFormat {
        id: 120_635_955,
        source_type: SourceType::Wikidata,
        name: "Microsoft Data Access Page",
        extensions: &["htm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
