use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129418045: FileType = FileType {
    file_format: &FileFormat {
        id: 129_418_045,
        source_type: SourceType::Wikidata,
        name: "GoodData-CL script file",
        extensions: &["gdc"],
        media_types: &["text/x-gooddata-cl"],
        signatures: &[],
        related_formats: &[],
    },
};
