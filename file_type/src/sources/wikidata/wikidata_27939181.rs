use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27939181: FileType = FileType {
    file_format: &FileFormat {
        id: 27_939_181,
        source_type: SourceType::Wikidata,
        name: "Enhanced Compression Wavelet, version 3",
        extensions: &["ecw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
