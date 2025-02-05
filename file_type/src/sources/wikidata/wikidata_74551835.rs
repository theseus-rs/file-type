use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74551835: FileFormat = FileFormat {
    id: 74_551_835,
    source_type: SourceType::Wikidata,
    name: "ChiWriter Screen Font",
    extensions: &["sft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
