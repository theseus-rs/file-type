use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967127: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_127,
        source_type: SourceType::Wikidata,
        name: "CMS",
        extensions: &["cms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
