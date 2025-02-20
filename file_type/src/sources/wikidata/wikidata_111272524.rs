use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272524: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_524,
        source_type: SourceType::Wikidata,
        name: "Ensoniq instrument file",
        extensions: &["efx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
