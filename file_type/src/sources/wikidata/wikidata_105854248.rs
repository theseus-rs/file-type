use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854248: FileFormat = FileFormat {
    id: 105_854_248,
    source_type: SourceType::Wikidata,
    name: "Elcomsoft Password Recovery config",
    extensions: &["axr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x63, 0x6F, 0x6D, 0x6D, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A, 0x66, 0x69, 0x6C,
                    0x65, 0x6E, 0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
