use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124663506: FileType = FileType {
    file_format: &FileFormat {
        id: 124_663_506,
        source_type: SourceType::Wikidata,
        name: "Transmission X-Ray Microscopy data format",
        extensions: &["txm", "txrm", "xrm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
