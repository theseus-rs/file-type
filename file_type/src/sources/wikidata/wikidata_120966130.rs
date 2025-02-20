use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120966130: FileType = FileType {
    file_format: &FileFormat {
        id: 120_966_130,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 97 data",
        extensions: &["mn5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
