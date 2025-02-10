use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34745761: FileType = FileType {
    file_format: &FileFormat {
        id: 34_745_761,
        source_type: SourceType::Wikidata,
        name: "StarCraft group file",
        extensions: &["grp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
