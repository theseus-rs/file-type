use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27866121: FileFormat = FileFormat {
    id: 27_866_121,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator Artwork file format, version 7.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    signatures: &[],
    related_formats: &[],
};
