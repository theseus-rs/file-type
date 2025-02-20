use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120920682: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_682,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2004 backup data",
        extensions: &["m12"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
