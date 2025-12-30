use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137196163: FileType = FileType {
    file_format: &FileFormat {
        id: 137_196_163,
        source_type: SourceType::Wikidata,
        name: "Minecraft Schematic File",
        extensions: &["schematic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
