use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113501336: FileType = FileType {
    file_format: &FileFormat {
        id: 113_501_336,
        source_type: SourceType::Wikidata,
        name: "PageMaker Mac Document 6.5-7.0",
        extensions: &["p65", "pmd", "pmt", "t65"],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
