use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27866113: FileFormat = FileFormat {
    id: 27_866_113,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork, version 4.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
