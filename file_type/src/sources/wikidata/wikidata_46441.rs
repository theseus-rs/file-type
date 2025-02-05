use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46441: FileFormat = FileFormat {
    id: 46_441,
    source_type: SourceType::Wikidata,
    name: "Cascading Style Sheets",
    extensions: &["css"],
    media_types: &["text/css"],
    signatures: &[],
    related_formats: &[],
};
