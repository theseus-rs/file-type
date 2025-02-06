use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860262: FileFormat = FileFormat {
    id: 105_860_262,
    source_type: SourceType::Wikidata,
    name: "RadDeveloper color scheme",
    extensions: &["rcs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x65, 0x78, 0x74, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x45, 0x57, 0x7C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
