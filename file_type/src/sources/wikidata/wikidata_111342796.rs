use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342796: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_796,
        source_type: SourceType::Wikidata,
        name: "Roland D-50 patch SysEx dump",
        extensions: &["syx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
