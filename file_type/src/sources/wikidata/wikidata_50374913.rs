use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50374913: FileType = FileType {
    file_format: &FileFormat {
        id: 50_374_913,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Design Web Format",
        extensions: &["dwfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
