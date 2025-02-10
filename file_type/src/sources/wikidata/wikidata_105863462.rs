use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863462: FileFormat = FileFormat {
    id: 105_863_462,
    source_type: SourceType::Wikidata,
    name: "KiCad Module library",
    extensions: &["emp", "mod"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x42, 0x4E, 0x45, 0x57, 0x2D, 0x4C, 0x69, 0x62, 0x4D, 0x6F, 0x64,
                    0x75, 0x6C, 0x65, 0x2D, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
