use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850289: FileFormat = FileFormat {
    id: 105_850_289,
    source_type: SourceType::Wikidata,
    name: "Carrara Environment",
    extensions: &["car"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x43, 0x20, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
