use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826390: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_390,
        source_type: SourceType::Wikidata,
        name: "Proxy AUX file, AUX.XML variant",
        extensions: &["aux.xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
