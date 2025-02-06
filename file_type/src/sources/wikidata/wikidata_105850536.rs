use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850536: FileFormat = FileFormat {
    id: 105_850_536,
    source_type: SourceType::Wikidata,
    name: "NBI Legend Chapter",
    extensions: &["chp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x56, 0x45, 0x20, 0x23, 0x20, 0x32, 0x2E, 0x30, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
