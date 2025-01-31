use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862249: FileFormat = FileFormat {
    id: 105_862_249,
    puid: "wikidata/105862249",
    name: "Musicline module",
    extensions: &["ml"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4C, 0x45, 0x44, 0x4D, 0x4F, 0x44, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
