use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979382: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_382,
        source_type: SourceType::Wikidata,
        name: "MPLS",
        extensions: &["mpl", "mpls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
