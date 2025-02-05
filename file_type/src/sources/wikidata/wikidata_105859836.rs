use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859836: FileFormat = FileFormat {
    id: 105_859_836,
    source_type: SourceType::Wikidata,
    name: "Valve Source BSP format (generic)",
    extensions: &["bsp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x42, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
