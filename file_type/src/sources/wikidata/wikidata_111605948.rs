use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111605948: FileFormat = FileFormat {
    id: 111_605_948,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2020",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
