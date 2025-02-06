use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866967: FileFormat = FileFormat {
    id: 105_866_967,
    source_type: SourceType::Wikidata,
    name: "Neko bytecode",
    extensions: &["n"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x45, 0x4B, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
