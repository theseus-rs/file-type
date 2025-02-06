use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858835: FileFormat = FileFormat {
    id: 105_858_835,
    source_type: SourceType::Wikidata,
    name: "Blowfish Advanced CS encrypted",
    extensions: &["bfa"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x08, 0x19, 0x92, 0x23, 0x00, 0x15, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
