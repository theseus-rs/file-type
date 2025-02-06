use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864728: FileFormat = FileFormat {
    id: 105_864_728,
    source_type: SourceType::Wikidata,
    name: "Persistence of Vision state",
    extensions: &["pov-state"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4F, 0x56, 0x2D, 0x52, 0x61, 0x79, 0x20, 0x52, 0x65, 0x6E, 0x64, 0x65,
                    0x72, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
