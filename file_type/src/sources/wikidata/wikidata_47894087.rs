use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47894087: FileType = FileType {
    file_format: &FileFormat {
        id: 47_894_087,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel ODBC Query",
        extensions: &["dqy"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
