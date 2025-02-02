use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866110: FileFormat = FileFormat {
    id: 27_866_110,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 8.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
