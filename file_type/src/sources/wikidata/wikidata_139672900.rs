use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139672900: FileType = FileType {
    file_format: &FileFormat {
        id: 139_672_900,
        source_type: SourceType::Wikidata,
        name: "osu! skin archive (.osk)",
        extensions: &["osk"],
        media_types: &["application/x-osu-skin-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
