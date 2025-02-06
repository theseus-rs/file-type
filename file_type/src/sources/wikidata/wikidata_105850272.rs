use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850272: FileFormat = FileFormat {
    id: 105_850_272,
    source_type: SourceType::Wikidata,
    name: "CAZIP compressed file",
    extensions: &["caz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x1A, 0x43, 0x41, 0x5A, 0x49, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
