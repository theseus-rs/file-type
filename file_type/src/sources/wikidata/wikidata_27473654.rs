use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473654: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_654,
        source_type: SourceType::Wikidata,
        name: "Band Interleaved by Pixel Image File",
        extensions: &["bip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
