use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856815: FileFormat = FileFormat {
    id: 105_856_815,
    puid: "wikidata/105856815",
    name: "Jinxter game save (PC)",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x77, 0x6A, 0x50, 0x61, 0x88, 0x91, 0x50, 0x61, 0x77, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
