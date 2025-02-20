use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111588248: FileType = FileType {
    file_format: &FileFormat {
        id: 111_588_248,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Interchange Document",
        extensions: &["inx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
