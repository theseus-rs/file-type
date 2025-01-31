use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864009: FileFormat = FileFormat {
    id: 105_864_009,
    puid: "wikidata/105864009",
    name: "MyLittleBase database",
    extensions: &["mlb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4C, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
