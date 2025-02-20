use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_77045990: FileType = FileType {
    file_format: &FileFormat {
        id: 77_045_990,
        source_type: SourceType::Wikidata,
        name: "Extensible 3D vector graphics (binary)",
        extensions: &["x3db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
