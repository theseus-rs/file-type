use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123014246: FileFormat = FileFormat {
    id: 123_014_246,
    source_type: SourceType::Wikidata,
    name: "PostScript 2.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
