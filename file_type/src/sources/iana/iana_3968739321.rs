use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3968739321: FileType = FileType {
    file_format: &FileFormat {
        id: 3_968_739_321,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-mbms-usage-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-mbms-usage-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
