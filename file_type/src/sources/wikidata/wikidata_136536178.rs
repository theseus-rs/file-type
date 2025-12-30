use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136536178: FileType = FileType {
    file_format: &FileFormat {
        id: 136_536_178,
        source_type: SourceType::Wikidata,
        name: "Microsoft Paint Project File",
        extensions: &["paint"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
