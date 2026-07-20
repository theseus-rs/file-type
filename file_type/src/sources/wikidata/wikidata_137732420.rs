use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137732420: FileType = FileType {
    file_format: &FileFormat {
        id: 137_732_420,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate project file",
        extensions: &["cp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
