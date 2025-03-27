use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27996279: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_279,
        source_type: SourceType::Wikidata,
        name: "MOBI",
        extensions: &["mobi", "prc"],
        media_types: &["application/x-mobipocket-ebook"],
        signatures: &[],
        related_formats: &[],
    },
};
