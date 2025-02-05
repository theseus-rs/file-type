use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849708: FileFormat = FileFormat {
    id: 105_849_708,
    source_type: SourceType::Wikidata,
    name: "Agat Emulator Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD4, 0x67, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
