use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119164692: FileType = FileType {
    file_format: &FileFormat {
        id: 119_164_692,
        source_type: SourceType::Wikidata,
        name: "ZoomText Configuration File",
        extensions: &["zxc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
