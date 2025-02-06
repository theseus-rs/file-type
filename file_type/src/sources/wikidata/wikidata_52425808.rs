use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52425808: FileFormat = FileFormat {
    id: 52_425_808,
    source_type: SourceType::Wikidata,
    name: "Vista Pro Graphics",
    extensions: &["dem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
