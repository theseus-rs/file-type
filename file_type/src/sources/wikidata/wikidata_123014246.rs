use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123014246: FileFormat = FileFormat {
    id: 123_014_246,
    source_type: SourceType::Wikidata,
    name: "PostScript 2.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
