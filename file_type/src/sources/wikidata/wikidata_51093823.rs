use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51093823: FileType = FileType {
    file_format: &FileFormat {
        id: 51_093_823,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Plot Configuration File, version R14",
        extensions: &["pc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
