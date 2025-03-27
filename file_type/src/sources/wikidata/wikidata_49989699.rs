use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49989699: FileType = FileType {
    file_format: &FileFormat {
        id: 49_989_699,
        source_type: SourceType::Wikidata,
        name: "VivoActive file format",
        extensions: &["viv"],
        media_types: &["video/vnd.vivo"],
        signatures: &[],
        related_formats: &[],
    },
};
