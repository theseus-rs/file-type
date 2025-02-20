use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2159: FileType = FileType {
    file_format: &FileFormat {
        id: 2_159,
        source_type: SourceType::Pronom,
        name: "Associated Signature Container Simple (ASiC-S)",
        extensions: &["asics", "scs"],
        media_types: &["application/vnd.etsi.asic-s+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
