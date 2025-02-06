use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858466: FileFormat = FileFormat {
    id: 105_858_466,
    source_type: SourceType::Wikidata,
    name: "The Elder Scrolls IV: Oblivion savegame",
    extensions: &["ess"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x45, 0x53, 0x34, 0x53, 0x41, 0x56, 0x45, 0x47, 0x41, 0x4D, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
