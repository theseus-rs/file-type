use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60662390: FileType = FileType {
    file_format: &FileFormat {
        id: 60_662_390,
        source_type: SourceType::Wikidata,
        name: "Inkwriter Template",
        extensions: &["pdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
