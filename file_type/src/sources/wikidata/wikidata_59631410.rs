use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59631410: FileFormat = FileFormat {
    id: 59_631_410,
    source_type: SourceType::Wikidata,
    name: "Navisworks Document",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
