use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866114: FileFormat = FileFormat {
    id: 27_866_114,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 5.0/5.5",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
