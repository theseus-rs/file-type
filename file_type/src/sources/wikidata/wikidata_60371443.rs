use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60371443: FileType = FileType {
    file_format: &FileFormat {
        id: 60_371_443,
        source_type: SourceType::Wikidata,
        name: "Quark Xpress Report File",
        extensions: &["qxp_report"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
