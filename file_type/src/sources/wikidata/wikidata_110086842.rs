use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110086842: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_842,
        source_type: SourceType::Wikidata,
        name: "Agisoft Point Cloud",
        extensions: &["oc3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
