use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117843186: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_186,
        source_type: SourceType::Wikidata,
        name: "Calculus EZ-Fax file",
        extensions: &["ezf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
