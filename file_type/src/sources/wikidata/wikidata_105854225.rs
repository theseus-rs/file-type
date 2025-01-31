use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854225: FileFormat = FileFormat {
    id: 105_854_225,
    puid: "wikidata/105854225",
    name: "Maxis XA Audio (music)",
    extensions: &["xa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x41, 0x4A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
