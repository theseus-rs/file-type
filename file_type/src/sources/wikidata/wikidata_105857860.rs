use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857860: FileFormat = FileFormat {
    id: 105_857_860,
    source_type: SourceType::Wikidata,
    name: "Sound Invasion Music System module",
    extensions: &["is"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x53, 0x4D, 0x21, 0x56, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
