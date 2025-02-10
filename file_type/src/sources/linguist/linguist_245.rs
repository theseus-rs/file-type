use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_245: FileType = FileType {
    file_format: &FileFormat {
        id: 245,
        source_type: SourceType::Linguist,
        name: "NetLinx+ERB",
        extensions: &["axi.erb", "axs.erb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
