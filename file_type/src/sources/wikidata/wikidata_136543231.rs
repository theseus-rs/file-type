use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136543231: FileType = FileType {
    file_format: &FileFormat {
        id: 136_543_231,
        source_type: SourceType::Wikidata,
        name: "Westwood Studios Animation",
        extensions: &["wsa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
