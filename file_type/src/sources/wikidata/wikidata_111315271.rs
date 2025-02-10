use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111315271: FileType = FileType {
    file_format: &FileFormat {
        id: 111_315_271,
        source_type: SourceType::Wikidata,
        name: "Gravis UltraSnd.ini bank setup",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
