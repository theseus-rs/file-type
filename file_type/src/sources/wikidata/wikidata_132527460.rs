use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132527460: FileType = FileType {
    file_format: &FileFormat {
        id: 132_527_460,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms EV3 Data Log file",
        extensions: &["rdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
