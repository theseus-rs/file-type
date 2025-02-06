use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857564: FileFormat = FileFormat {
    id: 105_857_564,
    source_type: SourceType::Wikidata,
    name: "IDle Object Library - bytecode",
    extensions: &["idol"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x64, 0x6C, 0x65, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
