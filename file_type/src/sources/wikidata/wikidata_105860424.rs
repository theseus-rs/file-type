use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860424: FileFormat = FileFormat {
    id: 105_860_424,
    source_type: SourceType::Wikidata,
    name: "Maxon Resource Creation Tool data",
    extensions: &["rct"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
