use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111395879: FileType = FileType {
    file_format: &FileFormat {
        id: 111_395_879,
        source_type: SourceType::Wikidata,
        name: "FloppyShot Image",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
