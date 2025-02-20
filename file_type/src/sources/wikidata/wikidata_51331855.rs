use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51331855: FileType = FileType {
    file_format: &FileFormat {
        id: 51_331_855,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint Design Template",
        extensions: &["pot"],
        media_types: &["application/mspowerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
