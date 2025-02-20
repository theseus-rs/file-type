use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116145260: FileType = FileType {
    file_format: &FileFormat {
        id: 116_145_260,
        source_type: SourceType::Wikidata,
        name: "FIT file",
        extensions: &["fit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
