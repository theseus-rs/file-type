use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2037645994: FileType = FileType {
    file_format: &FileFormat {
        id: 2_037_645_994,
        source_type: SourceType::Iana,
        name: "vnd.ga4gh.passport+jwt",
        extensions: &[],
        media_types: &["application/vnd.ga4gh.passport+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
