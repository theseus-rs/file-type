use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205983: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_983,
        source_type: SourceType::Wikidata,
        name: "Radiance Scene Description",
        extensions: &["rad"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
