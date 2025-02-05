use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_268086: FileFormat = FileFormat {
    id: 268_086,
    source_type: SourceType::Wikidata,
    name: "OMDoc",
    extensions: &["omdoc"],
    media_types: &["application/omdoc+xml"],
    signatures: &[],
    related_formats: &[],
};
