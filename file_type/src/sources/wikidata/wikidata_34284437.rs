use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34284437: FileType = FileType {
    file_format: &FileFormat {
        id: 34_284_437,
        source_type: SourceType::Wikidata,
        name: "Pascal script",
        extensions: &["inc", "p", "pas", "pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
