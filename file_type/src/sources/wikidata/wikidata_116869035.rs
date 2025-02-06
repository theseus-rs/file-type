use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116869035: FileFormat = FileFormat {
    id: 116_869_035,
    source_type: SourceType::Wikidata,
    name: "Summitsoft Business Card",
    extensions: &["crd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
