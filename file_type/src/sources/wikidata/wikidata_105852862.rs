use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852862: FileFormat = FileFormat {
    id: 105_852_862,
    source_type: SourceType::Wikidata,
    name: "Datel Max Drive (for GameCube) save state",
    extensions: &["sav"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x54, 0x45, 0x4C, 0x47, 0x43, 0x5F, 0x53, 0x41, 0x56, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
