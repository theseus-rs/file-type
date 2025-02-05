use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128584392: FileFormat = FileFormat {
    id: 128_584_392,
    source_type: SourceType::Wikidata,
    name: "ABNF file format",
    extensions: &["abnf"],
    media_types: &["text/x-abnf"],
    signatures: &[],
    related_formats: &[],
};
