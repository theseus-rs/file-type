use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757785: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_785,
        source_type: SourceType::Wikidata,
        name: "GMLJP2",
        extensions: &["jpf", "jpx"],
        media_types: &["image/jpx"],
        signatures: &[],
        related_formats: &[],
    },
};
