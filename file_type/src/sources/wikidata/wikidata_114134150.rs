use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114134150: FileType = FileType {
    file_format: &FileFormat {
        id: 114_134_150,
        source_type: SourceType::Wikidata,
        name: "MOPAC format",
        extensions: &["mop"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
