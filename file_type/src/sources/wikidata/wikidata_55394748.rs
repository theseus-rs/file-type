use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55394748: FileType = FileType {
    file_format: &FileFormat {
        id: 55_394_748,
        source_type: SourceType::Wikidata,
        name: "ChemDraw file",
        extensions: &["chm"],
        media_types: &["chemical/x-chemdraw"],
        signatures: &[],
        related_formats: &[],
    },
};
