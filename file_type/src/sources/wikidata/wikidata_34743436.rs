use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34743436: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_436,
        source_type: SourceType::Wikidata,
        name: "Softlib",
        extensions: &["slb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
