use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865341: FileFormat = FileFormat {
    id: 105_865_341,
    source_type: SourceType::Wikidata,
    name: "PC-Type document",
    extensions: &["pct"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xE0, 0x50, 0x43, 0x2D, 0x54, 0x79, 0x70, 0x65, 0x2B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
