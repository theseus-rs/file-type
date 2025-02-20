use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28344985: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_985,
        source_type: SourceType::Wikidata,
        name: "Genital Save State",
        extensions: &["gns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
