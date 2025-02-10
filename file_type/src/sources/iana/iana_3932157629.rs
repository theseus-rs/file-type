use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3932157629: FileType = FileType {
    file_format: &FileFormat {
        id: 3_932_157_629,
        source_type: SourceType::Iana,
        name: "font-tdpfr",
        extensions: &[],
        media_types: &["application/font-tdpfr"],
        signatures: &[],
        related_formats: &[],
    },
};
