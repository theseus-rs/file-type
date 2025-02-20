use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1196547: FileType = FileType {
    file_format: &FileFormat {
        id: 1_196_547,
        source_type: SourceType::Wikidata,
        name: "Design Web Format",
        extensions: &["dwf", "dwfx"],
        media_types: &["model/vnd-dwf"],
        signatures: &[],
        related_formats: &[],
    },
};
