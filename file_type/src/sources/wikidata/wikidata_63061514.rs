use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63061514: FileType = FileType {
    file_format: &FileFormat {
        id: 63_061_514,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word Document, version 97-2003",
        extensions: &["doc", "wbk"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
