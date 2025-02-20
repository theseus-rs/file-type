use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116790677: FileType = FileType {
    file_format: &FileFormat {
        id: 116_790_677,
        source_type: SourceType::Wikidata,
        name: "Prepress File",
        extensions: &["sep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
