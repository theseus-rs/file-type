use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_973167: FileType = FileType {
    file_format: &FileFormat {
        id: 973_167,
        source_type: SourceType::Wikidata,
        name: "NITF",
        extensions: &[],
        media_types: &["text/vnd.iptc.nitf"],
        signatures: &[],
        related_formats: &[],
    },
};
