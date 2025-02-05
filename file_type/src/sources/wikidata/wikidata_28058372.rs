use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28058372: FileFormat = FileFormat {
    id: 28_058_372,
    source_type: SourceType::Wikidata,
    name: "IFF-FAXX",
    extensions: &["faxx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
