use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863105: FileFormat = FileFormat {
    id: 105_863_105,
    source_type: SourceType::Wikidata,
    name: "TargetExpress target",
    extensions: &["mte"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x43, 0x57, 0x20, 0x54, 0x65, 0x63, 0x68, 0x6E, 0x6F, 0x67, 0x6F, 0x6C,
                    0x69, 0x65, 0x73, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x20, 0x54, 0x61, 0x72, 0x67,
                    0x65, 0x74, 0x45, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x66, 0x69, 0x6C,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
