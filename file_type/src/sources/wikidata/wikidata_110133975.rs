use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110133975: FileType = FileType {
    file_format: &FileFormat {
        id: 110_133_975,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2013",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
