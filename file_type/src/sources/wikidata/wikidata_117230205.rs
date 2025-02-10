use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117230205: FileType = FileType {
    file_format: &FileFormat {
        id: 117_230_205,
        source_type: SourceType::Wikidata,
        name: "PostScript file",
        extensions: &["ps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
