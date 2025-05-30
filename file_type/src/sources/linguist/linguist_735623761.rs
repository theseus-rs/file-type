use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_735623761: FileType = FileType {
    file_format: &FileFormat {
        id: 735_623_761,
        source_type: SourceType::Linguist,
        name: "Simple File Verification",
        extensions: &["sfv"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
