use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111665213: FileType = FileType {
    file_format: &FileFormat {
        id: 111_665_213,
        source_type: SourceType::Wikidata,
        name: "AbiWord Collaborative File Descriptor",
        extensions: &["abicollab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
