use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58960003: FileType = FileType {
    file_format: &FileFormat {
        id: 58_960_003,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Chart, version 3",
        extensions: &["slc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
