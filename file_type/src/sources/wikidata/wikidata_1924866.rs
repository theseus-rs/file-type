use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1924866: FileFormat = FileFormat {
    id: 1_924_866,
    source_type: SourceType::Wikidata,
    name: "Metalink",
    extensions: &["meta4", "metalink"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
