use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110134612: FileType = FileType {
    file_format: &FileFormat {
        id: 110_134_612,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2016",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
