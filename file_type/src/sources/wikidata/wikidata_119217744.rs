use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119217744: FileType = FileType {
    file_format: &FileFormat {
        id: 119_217_744,
        source_type: SourceType::Wikidata,
        name: "QuickBooks Accountant's Copy Work File",
        extensions: &["qba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
