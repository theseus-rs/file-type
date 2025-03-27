use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_68480845: FileType = FileType {
    file_format: &FileFormat {
        id: 68_480_845,
        source_type: SourceType::Wikidata,
        name: "DiscFerret disk image format",
        extensions: &["dfi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
