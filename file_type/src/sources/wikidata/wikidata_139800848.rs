use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139800848: FileType = FileType {
    file_format: &FileFormat {
        id: 139_800_848,
        source_type: SourceType::Wikidata,
        name: "TTPlayer Playlist",
        extensions: &["ttbl", "ttpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
