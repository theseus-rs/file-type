use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852824: FileFormat = FileFormat {
    id: 105_852_824,
    source_type: SourceType::Wikidata,
    name: "SuperTux Saved Game",
    extensions: &["stsg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x73, 0x75, 0x70, 0x65, 0x72, 0x74, 0x75, 0x78, 0x2D, 0x73, 0x61, 0x76,
                    0x65, 0x67, 0x61, 0x6D, 0x65, 0x0A, 0x20, 0x20, 0x28, 0x76, 0x65, 0x72, 0x73,
                    0x69, 0x6F, 0x6E, 0x20, 0x31, 0x29, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
