use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854508: FileFormat = FileFormat {
    id: 105_854_508,
    puid: "wikidata/105854508",
    name: "Recorded voice audio",
    extensions: &["zvr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x56, 0x52, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
