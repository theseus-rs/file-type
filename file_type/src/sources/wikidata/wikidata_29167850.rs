use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167850: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_850,
        source_type: SourceType::Wikidata,
        name: "P-touch Editor Label",
        extensions: &["lbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
