use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27939009: FileType = FileType {
    file_format: &FileFormat {
        id: 27_939_009,
        source_type: SourceType::Wikidata,
        name: "Enhanced Compression Wavelet, version 2",
        extensions: &["ecw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
