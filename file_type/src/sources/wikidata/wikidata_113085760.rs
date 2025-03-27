use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113085760: FileType = FileType {
    file_format: &FileFormat {
        id: 113_085_760,
        source_type: SourceType::Wikidata,
        name: "CB7",
        extensions: &["cb7"],
        media_types: &["application/x-cb7"],
        signatures: &[],
        related_formats: &[],
    },
};
