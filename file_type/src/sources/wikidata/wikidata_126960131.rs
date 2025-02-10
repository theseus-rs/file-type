use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126960131: FileType = FileType {
    file_format: &FileFormat {
        id: 126_960_131,
        source_type: SourceType::Wikidata,
        name: "Standard ML source code file",
        extensions: &["sml"],
        media_types: &["application/x-standardml", "text/x-standardml"],
        signatures: &[],
        related_formats: &[],
    },
};
