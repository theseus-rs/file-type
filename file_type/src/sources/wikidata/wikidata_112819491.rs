use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112819491: FileFormat = FileFormat {
    id: 112_819_491,
    source_type: SourceType::Wikidata,
    name: "Acclaim mocap file",
    extensions: &["amc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
