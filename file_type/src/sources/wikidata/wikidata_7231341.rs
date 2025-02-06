use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7231341: FileFormat = FileFormat {
    id: 7_231_341,
    source_type: SourceType::Wikidata,
    name: "Portable Database Image",
    extensions: &["pdi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
