use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858578: FileFormat = FileFormat {
    id: 105_858_578,
    puid: "wikidata/105858578",
    name: "CHDK UBASIC script (with rem)",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x65, 0x6D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
