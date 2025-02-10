use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125704723: FileType = FileType {
    file_format: &FileFormat {
        id: 125_704_723,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Master Document",
        extensions: &["sxg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
