use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121093219: FileType = FileType {
    file_format: &FileFormat {
        id: 121_093_219,
        source_type: SourceType::Wikidata,
        name: "Punch! Home Suite PSH file",
        extensions: &["psh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
