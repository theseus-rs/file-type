use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123014263: FileFormat = FileFormat {
    id: 123_014_263,
    source_type: SourceType::Wikidata,
    name: "PostScript 3.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
