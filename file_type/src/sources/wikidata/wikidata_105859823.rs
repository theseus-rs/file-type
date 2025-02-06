use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859823: FileFormat = FileFormat {
    id: 105_859_823,
    source_type: SourceType::Wikidata,
    name: "Emergency 3D model",
    extensions: &["v3o"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x56, 0x4E, 0x55, 0x4D, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
