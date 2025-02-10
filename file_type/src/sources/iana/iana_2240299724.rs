use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2240299724: FileType = FileType {
    file_format: &FileFormat {
        id: 2_240_299_724,
        source_type: SourceType::Iana,
        name: "vnd.cns.anp1",
        extensions: &[],
        media_types: &["audio/vnd.cns.anp1"],
        signatures: &[],
        related_formats: &[],
    },
};
