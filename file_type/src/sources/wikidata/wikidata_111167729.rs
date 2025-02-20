use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111167729: FileType = FileType {
    file_format: &FileFormat {
        id: 111_167_729,
        source_type: SourceType::Wikidata,
        name: "ACD/HNMR Calculated Spectrum file",
        extensions: &["hsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
