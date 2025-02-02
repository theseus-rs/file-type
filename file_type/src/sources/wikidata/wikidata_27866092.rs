use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866092: FileFormat = FileFormat {
    id: 27_866_092,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 1.0/1.1",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
