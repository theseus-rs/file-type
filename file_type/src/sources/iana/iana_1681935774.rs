use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1681935774: FileType = FileType {
    file_format: &FileFormat {
        id: 1_681_935_774,
        source_type: SourceType::Iana,
        name: "vnd.si.uricatalogue (OBSOLETED by request)",
        extensions: &[],
        media_types: &["text/vnd.si.uricatalogue"],
        signatures: &[],
        related_formats: &[],
    },
};
