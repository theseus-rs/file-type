use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58959780: FileType = FileType {
    file_format: &FileFormat {
        id: 58_959_780,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Chart, version 2",
        extensions: &["xlc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
