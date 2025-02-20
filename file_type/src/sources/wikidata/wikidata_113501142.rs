use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113501142: FileType = FileType {
    file_format: &FileFormat {
        id: 113_501_142,
        source_type: SourceType::Wikidata,
        name: "Cintel Raw Image",
        extensions: &["cri", "dvcc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
