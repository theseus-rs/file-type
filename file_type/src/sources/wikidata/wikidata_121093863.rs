use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121093863: FileType = FileType {
    file_format: &FileFormat {
        id: 121_093_863,
        source_type: SourceType::Wikidata,
        name: "Punch! Home Suite PHD file",
        extensions: &["phd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
