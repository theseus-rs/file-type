use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34305098: FileFormat = FileFormat {
    id: 34_305_098,
    source_type: SourceType::Wikidata,
    name: "Sassy Cascading Style Sheets",
    extensions: &["scss"],
    media_types: &["text/x-scss"],
    signatures: &[],
    related_formats: &[],
};
