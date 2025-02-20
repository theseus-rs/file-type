use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_827039157: FileType = FileType {
    file_format: &FileFormat {
        id: 827_039_157,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.TXF",
        extensions: &[],
        media_types: &["application/vnd.Mobius.TXF"],
        signatures: &[],
        related_formats: &[],
    },
};
