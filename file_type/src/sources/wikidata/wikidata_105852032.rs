use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852032: FileFormat = FileFormat {
    id: 105_852_032,
    puid: "wikidata/105852032",
    name: "SerialBox serials numbers package",
    extensions: &["sb2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x42, 0x6F, 0x78, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
