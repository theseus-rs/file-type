use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113162744: FileType = FileType {
    file_format: &FileFormat {
        id: 113_162_744,
        source_type: SourceType::Wikidata,
        name: "MyDeluxeInvoices & Estimates file",
        extensions: &["inv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
