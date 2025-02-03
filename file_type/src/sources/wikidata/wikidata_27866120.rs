use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866120: FileFormat = FileFormat {
    id: 27_866_120,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 6.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
