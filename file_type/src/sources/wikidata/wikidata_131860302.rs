use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131860302: FileType = FileType {
    file_format: &FileFormat {
        id: 131_860_302,
        source_type: SourceType::Wikidata,
        name: "MNI transformation file format",
        extensions: &["xfm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
