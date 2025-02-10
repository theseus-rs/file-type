use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206553: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_553,
        source_type: SourceType::Wikidata,
        name: "MAKIchan Graphics MKI",
        extensions: &["mag", "max", "mki"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
