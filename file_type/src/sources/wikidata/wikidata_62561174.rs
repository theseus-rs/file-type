use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62561174: FileType = FileType {
    file_format: &FileFormat {
        id: 62_561_174,
        source_type: SourceType::Wikidata,
        name: "Pagemaker Document",
        extensions: &["p65", "pmd", "pmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
