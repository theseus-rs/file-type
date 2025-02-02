use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861025: FileFormat = FileFormat {
    id: 105_861_025,
    source_type: SourceType::Wikidata,
    name: "WinWay Letter (v4.0)",
    extensions: &["ltr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0E, 0x00, 0x01, 0x00, 0x26, 0x00, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40,
                    0x40, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x08,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
