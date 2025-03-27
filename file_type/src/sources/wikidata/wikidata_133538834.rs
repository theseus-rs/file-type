use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133538834: FileType = FileType {
    file_format: &FileFormat {
        id: 133_538_834,
        source_type: SourceType::Wikidata,
        name: "DaVinci file",
        extensions: &["img"],
        media_types: &["image/x-davinci"],
        signatures: &[],
        related_formats: &[],
    },
};
