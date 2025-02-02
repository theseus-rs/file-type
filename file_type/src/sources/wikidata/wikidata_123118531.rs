use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123118531: FileFormat = FileFormat {
    id: 123_118_531,
    source_type: SourceType::Wikidata,
    name: "PostScript 3.1",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
