use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852797: FileFormat = FileFormat {
    id: 105_852_797,
    source_type: SourceType::Wikidata,
    name: "VIVA Story",
    extensions: &["story"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x49, 0x56, 0x41, 0x20, 0x41, 0x4D, 0x49, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
