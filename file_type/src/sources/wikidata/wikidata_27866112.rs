use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866112: FileFormat = FileFormat {
    id: 27_866_112,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 3.0/3.2",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
