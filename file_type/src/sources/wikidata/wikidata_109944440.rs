use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109944440: FileFormat = FileFormat {
    id: 109_944_440,
    source_type: SourceType::Wikidata,
    name: "CadKey file format",
    extensions: &["prt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
