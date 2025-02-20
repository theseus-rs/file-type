use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29896310: FileType = FileType {
    file_format: &FileFormat {
        id: 29_896_310,
        source_type: SourceType::Wikidata,
        name: "ABIF",
        extensions: &["ab1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
