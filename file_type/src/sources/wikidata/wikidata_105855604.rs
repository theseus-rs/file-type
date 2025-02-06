use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855604: FileFormat = FileFormat {
    id: 105_855_604,
    source_type: SourceType::Wikidata,
    name: "EPOC OPL Object module",
    extensions: &["opo"],
    media_types: &["application/x-epoc-opo"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x00, 0x00, 0x10, 0x73, 0x00, 0x00, 0x10, 0x68, 0x01, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
