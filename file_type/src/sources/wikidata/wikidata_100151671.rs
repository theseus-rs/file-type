use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100151671: FileType = FileType {
    file_format: &FileFormat {
        id: 100_151_671,
        source_type: SourceType::Wikidata,
        name: "Bruker PDZ",
        extensions: &["pdz", "xpdz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
