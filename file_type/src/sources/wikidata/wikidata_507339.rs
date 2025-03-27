use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_507339: FileType = FileType {
    file_format: &FileFormat {
        id: 507_339,
        source_type: SourceType::Wikidata,
        name: "Web Application Description Language",
        extensions: &["wadl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
