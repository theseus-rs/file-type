use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826389: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_389,
        source_type: SourceType::Wikidata,
        name: "Proxy AUX file, AUX variant",
        extensions: &["aux"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
