use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51093476: FileType = FileType {
    file_format: &FileFormat {
        id: 51_093_476,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel OLAP Query",
        extensions: &["oqy"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
