use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206740: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_740,
        source_type: SourceType::Wikidata,
        name: "Navy Image File Format",
        extensions: &["ct3", "nif"],
        media_types: &["image/x-niff"],
        signatures: &[],
        related_formats: &[],
    },
};
