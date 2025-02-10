use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110132901: FileType = FileType {
    file_format: &FileFormat {
        id: 110_132_901,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2010",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
