use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850337: FileFormat = FileFormat {
    id: 105_850_337,
    source_type: SourceType::Wikidata,
    name: "Cabbage script (with rem)",
    extensions: &["csd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
