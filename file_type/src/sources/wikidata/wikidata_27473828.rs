use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473828: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_828,
        source_type: SourceType::Wikidata,
        name: "BIL/BIP/BSQ Color File",
        extensions: &["clr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
