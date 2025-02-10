use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111395876: FileType = FileType {
    file_format: &FileFormat {
        id: 111_395_876,
        source_type: SourceType::Wikidata,
        name: "Konica format",
        extensions: &["kqp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
