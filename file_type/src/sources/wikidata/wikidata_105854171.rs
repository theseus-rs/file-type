use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854171: FileFormat = FileFormat {
    id: 105_854_171,
    puid: "wikidata/105854171",
    name: "The Sims Archive",
    extensions: &["far"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x41, 0x52, 0x21, 0x62, 0x79, 0x41, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
