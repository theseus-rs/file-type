use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95999881: FileType = FileType {
    file_format: &FileFormat {
        id: 95_999_881,
        source_type: SourceType::Wikidata,
        name: "NDK seismographic data format",
        extensions: &["ndk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
