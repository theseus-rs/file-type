use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757684: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_684,
        source_type: SourceType::Wikidata,
        name: "Nintendo Game Boy cartridge SAV file",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
