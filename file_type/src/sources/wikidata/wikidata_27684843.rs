use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27684843: FileType = FileType {
    file_format: &FileFormat {
        id: 27_684_843,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 2.0",
        extensions: &["pub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
