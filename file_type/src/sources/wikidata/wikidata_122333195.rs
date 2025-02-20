use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122333195: FileType = FileType {
    file_format: &FileFormat {
        id: 122_333_195,
        source_type: SourceType::Wikidata,
        name: "HCK image file",
        extensions: &["hck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
