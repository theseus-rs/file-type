use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308880: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_880,
        source_type: SourceType::Wikidata,
        name: "DOS Sound and Music Interface Advanced Module Format",
        extensions: &["amf"],
        media_types: &["application/x-mod"],
        signatures: &[],
        related_formats: &[],
    },
};
