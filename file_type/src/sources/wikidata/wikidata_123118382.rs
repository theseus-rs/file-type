use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123118382: FileFormat = FileFormat {
    id: 123_118_382,
    source_type: SourceType::Wikidata,
    name: "PostScript 1.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
