use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1315297: FileType = FileType {
    file_format: &FileFormat {
        id: 1_315_297,
        source_type: SourceType::Wikidata,
        name: "QuickTime VR",
        extensions: &["qtvr"],
        media_types: &["video/quicktime"],
        signatures: &[],
        related_formats: &[],
    },
};
