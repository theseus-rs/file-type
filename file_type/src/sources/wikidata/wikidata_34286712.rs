use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34286712: FileType = FileType {
    file_format: &FileFormat {
        id: 34_286_712,
        source_type: SourceType::Wikidata,
        name: "Processing Development Environment",
        extensions: &["pde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
