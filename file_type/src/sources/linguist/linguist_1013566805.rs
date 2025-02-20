use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1013566805: FileType = FileType {
    file_format: &FileFormat {
        id: 1_013_566_805,
        source_type: SourceType::Linguist,
        name: "LTspice Symbol",
        extensions: &["asy"],
        media_types: &["text/x-spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};
