use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856020: FileFormat = FileFormat {
    id: 105_856_020,
    puid: "wikidata/105856020",
    name: "Norton Disk Doctor UnDo file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4E, 0x43, 0x49, 0x55, 0x4E, 0x44, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
