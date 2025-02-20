use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966975: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_975,
        source_type: SourceType::Wikidata,
        name: "WSR",
        extensions: &["wsr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
