use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857099: FileFormat = FileFormat {
    id: 105_857_099,
    source_type: SourceType::Wikidata,
    name: "Guitar Pro v5 tablature",
    extensions: &["gp5"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x46, 0x49, 0x43, 0x48, 0x49, 0x45, 0x52, 0x20, 0x47, 0x55, 0x49, 0x54,
                    0x41, 0x52, 0x20, 0x50, 0x52, 0x4F, 0x20, 0x76, 0x35, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
