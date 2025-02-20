use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58367950: FileType = FileType {
    file_format: &FileFormat {
        id: 58_367_950,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file format version 2007",
        extensions: &["mpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
