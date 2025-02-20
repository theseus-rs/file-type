use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2028473068: FileType = FileType {
    file_format: &FileFormat {
        id: 2_028_473_068,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-base+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-base+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
