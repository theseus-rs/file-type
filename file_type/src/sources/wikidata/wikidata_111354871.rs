use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111354871: FileType = FileType {
    file_format: &FileFormat {
        id: 111_354_871,
        source_type: SourceType::Wikidata,
        name: "Steinberg LM-4 banks",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
