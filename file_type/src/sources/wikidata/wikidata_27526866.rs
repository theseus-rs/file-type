use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27526866: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_866,
        source_type: SourceType::Wikidata,
        name: "Write for Windows Document, version 3.1",
        extensions: &["wri"],
        media_types: &["application/x-mswrite"],
        signatures: &[],
        related_formats: &[],
    },
};
