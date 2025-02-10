use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2338089597: FileType = FileType {
    file_format: &FileFormat {
        id: 2_338_089_597,
        source_type: SourceType::Iana,
        name: "scip",
        extensions: &[],
        media_types: &["video/scip"],
        signatures: &[],
        related_formats: &[],
    },
};
