use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858793: FileFormat = FileFormat {
    id: 105_858_793,
    source_type: SourceType::Wikidata,
    name: "PDS image bitmap",
    extensions: &["img", "vic"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x4A, 0x50, 0x4C, 0x31, 0x49, 0x30, 0x30, 0x50, 0x44, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
