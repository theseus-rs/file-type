use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27684912: FileType = FileType {
    file_format: &FileFormat {
        id: 27_684_912,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 11.0",
        extensions: &["pub"],
        media_types: &["application/vnd.ms-publisher"],
        signatures: &[],
        related_formats: &[],
    },
};
