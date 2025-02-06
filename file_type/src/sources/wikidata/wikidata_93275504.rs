use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_93275504: FileFormat = FileFormat {
    id: 93_275_504,
    source_type: SourceType::Wikidata,
    name: "Procreate",
    extensions: &["procreate"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
