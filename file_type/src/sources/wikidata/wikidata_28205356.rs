use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205356: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_356,
        source_type: SourceType::Wikidata,
        name: "K25",
        extensions: &["k25"],
        media_types: &["image/x-kodak-k25", "image/x-raw-kodak"],
        signatures: &[],
        related_formats: &[],
    },
};
