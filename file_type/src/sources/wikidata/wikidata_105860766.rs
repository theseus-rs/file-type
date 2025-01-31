use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860766: FileFormat = FileFormat {
    id: 105_860_766,
    puid: "wikidata/105860766",
    name: "MDL Reaction format",
    extensions: &["rxn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x24, 0x52, 0x58, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
