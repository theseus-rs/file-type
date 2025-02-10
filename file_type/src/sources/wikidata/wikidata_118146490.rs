use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118146490: FileType = FileType {
    file_format: &FileFormat {
        id: 118_146_490,
        source_type: SourceType::Wikidata,
        name: "Edge-couple Symmetric Stripline File",
        extensions: &["tl5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
