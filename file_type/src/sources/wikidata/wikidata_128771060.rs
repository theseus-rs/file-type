use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128771060: FileType = FileType {
    file_format: &FileFormat {
        id: 128_771_060,
        source_type: SourceType::Wikidata,
        name: "Chapel source code file",
        extensions: &["chpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
