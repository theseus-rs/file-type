use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110132623: FileType = FileType {
    file_format: &FileFormat {
        id: 110_132_623,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2007",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
