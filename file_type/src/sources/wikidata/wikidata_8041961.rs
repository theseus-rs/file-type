use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_8041961: FileFormat = FileFormat {
    id: 8_041_961,
    source_type: SourceType::Wikidata,
    name: "eXtensible Graph Markup and Modeling Language",
    extensions: &["XGMML"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
