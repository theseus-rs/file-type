use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3943569: FileFormat = FileFormat {
    id: 3_943_569,
    source_type: SourceType::Wikidata,
    name: "SEG-Y",
    extensions: &["segy", "sgy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
