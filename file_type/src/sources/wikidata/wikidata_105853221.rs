use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853221: FileFormat = FileFormat {
    id: 105_853_221,
    source_type: SourceType::Wikidata,
    name: "FreeDOS KEYBoard layout collection",
    extensions: &["sys"],
    media_types: &["application/x-fdos-keyb"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x43, 0x46, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
