use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130358240: FileType = FileType {
    file_format: &FileFormat {
        id: 130_358_240,
        source_type: SourceType::Wikidata,
        name: "Mscgen file format",
        extensions: &["msc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
