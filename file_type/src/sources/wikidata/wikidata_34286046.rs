use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34286046: FileType = FileType {
    file_format: &FileFormat {
        id: 34_286_046,
        source_type: SourceType::Wikidata,
        name: "Pixie script",
        extensions: &["pxi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
