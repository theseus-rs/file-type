use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44933672: FileType = FileType {
    file_format: &FileFormat {
        id: 44_933_672,
        source_type: SourceType::Wikidata,
        name: "dBASE IV file format",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
