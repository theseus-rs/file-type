use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858341: FileFormat = FileFormat {
    id: 105_858_341,
    source_type: SourceType::Wikidata,
    name: "Excelsior Phase Two game data archive",
    extensions: &["con", "dat", "def", "gfx", "map", "snd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x78, 0x32, 0x20, 0x4C, 0x69, 0x62, 0x20, 0x76, 0x31, 0x2E, 0x30, 0x20,
                    0x20, 0x20, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
