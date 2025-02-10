use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110131505: FileType = FileType {
    file_format: &FileFormat {
        id: 110_131_505,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2003",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
