use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866219: FileFormat = FileFormat {
    id: 105_866_219,
    source_type: SourceType::Wikidata,
    name: "Pixie vector graphic",
    extensions: &["pxi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x05, 0xDA, 0xA7, 0x49, 0x01, 0x2C, 0x99, 0x0B, 0xDC, 0x04, 0x10, 0x89,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
