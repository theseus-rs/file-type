use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27684898: FileType = FileType {
    file_format: &FileFormat {
        id: 27_684_898,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher file format, version 9.0",
        extensions: &["pub"],
        media_types: &["application/vnd.ms-publisher"],
        signatures: &[],
        related_formats: &[],
    },
};
