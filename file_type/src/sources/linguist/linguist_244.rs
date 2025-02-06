use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_244: FileFormat = FileFormat {
    id: 244,
    source_type: SourceType::Linguist,
    name: "NetLinx",
    extensions: &["axi", "axs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
