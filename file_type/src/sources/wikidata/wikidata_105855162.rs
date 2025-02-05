use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855162: FileFormat = FileFormat {
    id: 105_855_162,
    source_type: SourceType::Wikidata,
    name: "Psion fast Font",
    extensions: &["fon"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4E, 0x31, 0xC5, 0x10, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
