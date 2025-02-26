use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132636962: FileType = FileType {
    file_format: &FileFormat {
        id: 132_636_962,
        source_type: SourceType::Wikidata,
        name: "PureBasic source code file",
        extensions: &["pb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
