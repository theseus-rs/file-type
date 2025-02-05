use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95994878: FileFormat = FileFormat {
    id: 95_994_878,
    source_type: SourceType::Wikidata,
    name: "Canadian digital elevation data format",
    extensions: &["dem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
