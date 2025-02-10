use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28756196: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_196,
        source_type: SourceType::Wikidata,
        name: "FWKCS NDX file",
        extensions: &["ndx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
