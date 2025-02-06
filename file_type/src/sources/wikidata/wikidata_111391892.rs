use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111391892: FileFormat = FileFormat {
    id: 111_391_892,
    source_type: SourceType::Wikidata,
    name: "Bryce2 Object",
    extensions: &["obj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
