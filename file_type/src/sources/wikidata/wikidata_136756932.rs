use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136756932: FileType = FileType {
    file_format: &FileFormat {
        id: 136_756_932,
        source_type: SourceType::Wikidata,
        name: "Adobe XD file format",
        extensions: &["xd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
