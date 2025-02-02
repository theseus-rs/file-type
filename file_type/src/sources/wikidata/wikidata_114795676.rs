use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114795676: FileFormat = FileFormat {
    id: 114_795_676,
    source_type: SourceType::Wikidata,
    name: "ReadCube Enhanced PDF",
    extensions: &["epdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
