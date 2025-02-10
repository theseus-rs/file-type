use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_183169: FileType = FileType {
    file_format: &FileFormat {
        id: 183_169,
        source_type: SourceType::Wikidata,
        name: "Jakarta Server Pages",
        extensions: &["jsp"],
        media_types: &["application/jsp"],
        signatures: &[],
        related_formats: &[],
    },
};
