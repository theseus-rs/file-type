use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51718015: FileType = FileType {
    file_format: &FileFormat {
        id: 51_718_015,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel OLE DB Query",
        extensions: &["rqy"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
