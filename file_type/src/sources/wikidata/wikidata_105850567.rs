use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850567: FileFormat = FileFormat {
    id: 105_850_567,
    source_type: SourceType::Wikidata,
    name: "Total Commander CRC file",
    extensions: &["crc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x69, 0x6C, 0x65, 0x6E, 0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
