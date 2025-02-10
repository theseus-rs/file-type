use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111355400: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_400,
        source_type: SourceType::Wikidata,
        name: "Annotated speech file",
        extensions: &["vap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
