use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864005: FileFormat = FileFormat {
    id: 105_864_005,
    source_type: SourceType::Wikidata,
    name: "Mass Properties Exchange data",
    extensions: &["mpex", "txt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x50, 0x45, 0x58, 0x09, 0x44, 0x41, 0x54, 0x41, 0x53, 0x45, 0x54, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
