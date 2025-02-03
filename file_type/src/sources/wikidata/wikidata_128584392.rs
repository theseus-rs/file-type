use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128584392: FileFormat = FileFormat {
    id: 128_584_392,
    source_type: SourceType::Wikidata,
    name: "ABNF file format",
    extensions: &["abnf"],
    media_types: &["text/x-abnf"],
    internal_signatures: &[],
    related_formats: &[],
};
