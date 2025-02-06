use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600772: FileFormat = FileFormat {
    id: 28_600_772,
    source_type: SourceType::Wikidata,
    name: "EnCase hash map",
    extensions: &["EnMap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
