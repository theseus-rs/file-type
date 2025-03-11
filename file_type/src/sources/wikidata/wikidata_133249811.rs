use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133249811: FileType = FileType {
    file_format: &FileFormat {
        id: 133_249_811,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file, version 3",
        extensions: &["mpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
