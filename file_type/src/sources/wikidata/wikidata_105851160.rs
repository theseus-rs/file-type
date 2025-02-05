use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851160: FileFormat = FileFormat {
    id: 105_851_160,
    source_type: SourceType::Wikidata,
    name: "TRCZip compressed data",
    extensions: &["trc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xB0, 0xB1, 0xB2, 0x54, 0x52, 0x43, 0x5A, 0x69, 0x70, 0xB2, 0xB1, 0xB0,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
