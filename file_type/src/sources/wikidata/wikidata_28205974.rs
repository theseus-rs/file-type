use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205974: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_974,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Y Luminance Channel",
        extensions: &["imy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
