use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_575143428: FileFormat = FileFormat {
    id: 575_143_428,
    source_type: SourceType::Linguist,
    name: "ImageJ Macro",
    extensions: &["ijm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
