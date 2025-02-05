use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855413: FileFormat = FileFormat {
    id: 105_855_413,
    source_type: SourceType::Wikidata,
    name: "Foxit Reader Add-on",
    extensions: &["fzip"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x06, 0x78, 0x19, 0x01, 0x01, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
