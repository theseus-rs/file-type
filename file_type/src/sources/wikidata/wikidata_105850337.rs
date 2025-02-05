use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850337: FileFormat = FileFormat {
    id: 105_850_337,
    source_type: SourceType::Wikidata,
    name: "Cabbage script (with rem)",
    extensions: &["csd"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
