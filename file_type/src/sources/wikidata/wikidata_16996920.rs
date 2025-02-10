use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_16996920: FileType = FileType {
    file_format: &FileFormat {
        id: 16_996_920,
        source_type: SourceType::Wikidata,
        name: "Windows Setup Information File",
        extensions: &["inf"],
        media_types: &[
            "application/inf",
            "application/x-setupscript",
            "application/x-wine-extension-inf",
            "text/x-inf",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
