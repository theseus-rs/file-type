use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856509: FileFormat = FileFormat {
    id: 105_856_509,
    source_type: SourceType::Wikidata,
    name: "WLF WolfMAME recording info",
    extensions: &["wlf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x4C, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
