use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2544845359: FileType = FileType {
    file_format: &FileFormat {
        id: 2_544_845_359,
        source_type: SourceType::Iana,
        name: "vnd.cns.inf1",
        extensions: &[],
        media_types: &["audio/vnd.cns.inf1"],
        signatures: &[],
        related_formats: &[],
    },
};
