use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967108: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_108,
        source_type: SourceType::Wikidata,
        name: "STOS memory bank",
        extensions: &["mbk", "mbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
