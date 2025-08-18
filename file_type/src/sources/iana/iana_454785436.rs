use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_454785436: FileType = FileType {
    file_format: &FileFormat {
        id: 454_785_436,
        source_type: SourceType::Iana,
        name: "vnd.sirtx.vmv0",
        extensions: &[],
        media_types: &["application/vnd.sirtx.vmv0"],
        signatures: &[],
        related_formats: &[],
    },
};
