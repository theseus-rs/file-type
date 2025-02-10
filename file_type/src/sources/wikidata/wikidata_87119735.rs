use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87119735: FileType = FileType {
    file_format: &FileFormat {
        id: 87_119_735,
        source_type: SourceType::Wikidata,
        name: "Open Financial Exchange 1.03",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[],
        related_formats: &[],
    },
};
