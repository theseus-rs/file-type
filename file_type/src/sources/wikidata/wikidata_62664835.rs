use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62664835: FileType = FileType {
    file_format: &FileFormat {
        id: 62_664_835,
        source_type: SourceType::Wikidata,
        name: "Active Server Page",
        extensions: &["asp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
