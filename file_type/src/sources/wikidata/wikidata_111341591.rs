use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341591: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_591,
        source_type: SourceType::Wikidata,
        name: "EMU SoundFont v1.0 bank",
        extensions: &["sbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
