use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849726: FileFormat = FileFormat {
    id: 105_849_726,
    source_type: SourceType::Wikidata,
    name: "Codecrypt ASCII-armored format",
    extensions: &["ccr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x63, 0x63, 0x72, 0x20, 0x62, 0x65, 0x67,
                    0x69, 0x6E, 0x20, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
