use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205367: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_367,
        source_type: SourceType::Wikidata,
        name: "DCR",
        extensions: &["dcr"],
        media_types: &["image/x-kodak-dcr", "image/x-raw-kodak"],
        signatures: &[],
        related_formats: &[],
    },
};
