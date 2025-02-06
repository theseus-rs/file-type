use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944440: FileFormat = FileFormat {
    id: 109_944_440,
    source_type: SourceType::Wikidata,
    name: "CadKey file format",
    extensions: &["prt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
