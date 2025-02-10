use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130288276: FileType = FileType {
    file_format: &FileFormat {
        id: 130_288_276,
        source_type: SourceType::Wikidata,
        name: "MYTHSAV",
        extensions: &["mythsav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
