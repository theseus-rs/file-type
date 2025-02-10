use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167417: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_417,
        source_type: SourceType::Wikidata,
        name: "Folio",
        extensions: &["folio"],
        media_types: &["application/vnd.adobe.folio+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
