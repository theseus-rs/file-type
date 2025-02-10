use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_56827141: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_141,
        source_type: SourceType::Wikidata,
        name: "SeqBox file",
        extensions: &["sbx"],
        media_types: &["application/x-sbx"],
        signatures: &[],
        related_formats: &[],
    },
};
