use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849843: FileFormat = FileFormat {
    id: 105_849_843,
    source_type: SourceType::Wikidata,
    name: "ExpressCalc spreadsheet (v4.0)",
    extensions: &["cal"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x43, 0x20, 0x34, 0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
