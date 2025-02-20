use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130280165: FileType = FileType {
    file_format: &FileFormat {
        id: 130_280_165,
        source_type: SourceType::Wikidata,
        name: "Mask file format",
        extensions: &["mask"],
        media_types: &["text/x-mask"],
        signatures: &[],
        related_formats: &[],
    },
};
