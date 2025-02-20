use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111440765: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_765,
        source_type: SourceType::Wikidata,
        name: "Ruby source code",
        extensions: &["rb"],
        media_types: &["application/x-ruby", "text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
