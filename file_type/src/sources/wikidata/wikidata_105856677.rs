use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856677: FileFormat = FileFormat {
    id: 105_856_677,
    puid: "wikidata/105856677",
    name: "UNIMod created by APlayer",
    extensions: &["uni"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50, 0x55, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
