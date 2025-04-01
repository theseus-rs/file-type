use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7978505: FileType = FileType {
    file_format: &FileFormat {
        id: 7_978_505,
        source_type: SourceType::Wikidata,
        name: "Web ARChive",
        extensions: &["warc", "warc.gz"],
        media_types: &["application/warc"],
        signatures: &[],
        related_formats: &[],
    },
};
