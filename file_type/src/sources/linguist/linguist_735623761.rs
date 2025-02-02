use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_735623761: FileFormat = FileFormat {
    id: 735_623_761,
    source_type: SourceType::Linguist,
    name: "Simple File Verification",
    extensions: &["sfv"],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
