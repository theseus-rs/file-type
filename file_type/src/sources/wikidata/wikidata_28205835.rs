use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205835: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_835,
        source_type: SourceType::Wikidata,
        name: "Cloé picture",
        extensions: &["clo", "cloe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
