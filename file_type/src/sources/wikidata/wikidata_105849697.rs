use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849697: FileFormat = FileFormat {
    id: 105_849_697,
    source_type: SourceType::Wikidata,
    name: "Cramfs ROM filesystem package (big endian)",
    extensions: &["cmg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0xCD, 0x3D, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
