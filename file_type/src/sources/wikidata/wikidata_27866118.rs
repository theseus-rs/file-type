use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866118: FileFormat = FileFormat {
    id: 27_866_118,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 5.x Japenese Edition",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
