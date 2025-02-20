use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111167713: FileType = FileType {
    file_format: &FileFormat {
        id: 111_167_713,
        source_type: SourceType::Wikidata,
        name: "ACD/CNMR Calculated Spectrum file",
        extensions: &["csp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
