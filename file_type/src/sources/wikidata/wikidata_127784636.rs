use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127784636: FileType = FileType {
    file_format: &FileFormat {
        id: 127_784_636,
        source_type: SourceType::Wikidata,
        name: "Metal Shading Language File",
        extensions: &["metal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
