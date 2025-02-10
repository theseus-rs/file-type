use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48814699: FileType = FileType {
    file_format: &FileFormat {
        id: 48_814_699,
        source_type: SourceType::Wikidata,
        name: "DesignCAD for Windows Drawing",
        extensions: &["dw2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
