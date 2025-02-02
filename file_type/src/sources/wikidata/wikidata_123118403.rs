use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123118403: FileFormat = FileFormat {
    id: 123_118_403,
    source_type: SourceType::Wikidata,
    name: "PostScript 2.1",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
