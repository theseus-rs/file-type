use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167417: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_417,
        source_type: SourceType::Wikidata,
        name: "Q29167417",
        extensions: &["folio"],
        media_types: &["application/vnd.adobe.folio+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
