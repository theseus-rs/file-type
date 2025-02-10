use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_109053247: FileType = FileType {
    file_format: &FileFormat {
        id: 109_053_247,
        source_type: SourceType::Iana,
        name: "vnd.collabio.xodocuments.spreadsheet",
        extensions: &[],
        media_types: &["application/vnd.collabio.xodocuments.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};
