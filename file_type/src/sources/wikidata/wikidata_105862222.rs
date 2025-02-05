use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862222: FileFormat = FileFormat {
    id: 105_862_222,
    source_type: SourceType::Wikidata,
    name: "Magnetic Graphics (V2)",
    extensions: &["gfx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x50, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
