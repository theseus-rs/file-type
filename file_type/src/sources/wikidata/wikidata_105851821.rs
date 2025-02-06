use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851821: FileFormat = FileFormat {
    id: 105_851_821,
    source_type: SourceType::Wikidata,
    name: "Artline Symbol File",
    extensions: &["syf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x43, 0x50, 0x41, 0x30, 0x30, 0x32, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
