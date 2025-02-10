use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_667863728: FileType = FileType {
    file_format: &FileFormat {
        id: 667_863_728,
        source_type: SourceType::Iana,
        name: "vnd.etsi.overload-control-policy-dataset+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.overload-control-policy-dataset+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
