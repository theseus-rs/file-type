use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205619: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_619,
        source_type: SourceType::Wikidata,
        name: "RIPscrip version 2 Hot Icon",
        extensions: &["bmh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
