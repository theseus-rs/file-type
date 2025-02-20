use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27826343: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_343,
        source_type: SourceType::Wikidata,
        name: "Auxiliary file, AUX.XML variant",
        extensions: &["aux.xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
