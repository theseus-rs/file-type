use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7903498: FileFormat = FileFormat {
    id: 7_903_498,
    source_type: SourceType::Wikidata,
    name: "UTZ",
    extensions: &["utz"],
    media_types: &["application/vnd.uiq.theme"],
    signatures: &[],
    related_formats: &[],
};
