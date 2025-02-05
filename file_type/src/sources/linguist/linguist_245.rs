use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_245: FileFormat = FileFormat {
    id: 245,
    source_type: SourceType::Linguist,
    name: "NetLinx+ERB",
    extensions: &["axi.erb", "axs.erb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
