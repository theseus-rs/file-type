use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27473691: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_691,
        source_type: SourceType::Wikidata,
        name: "BIL/BIP/BSQ Header File",
        extensions: &["hdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
