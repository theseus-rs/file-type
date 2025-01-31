use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858923: FileFormat = FileFormat {
    id: 105_858_923,
    puid: "wikidata/105858923",
    name: "Buzzic 2 module",
    extensions: &["buz2"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x75, 0x7A, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
