use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858625: FileFormat = FileFormat {
    id: 105_858_625,
    source_type: SourceType::Wikidata,
    name: "Nintendo GameCube/Wii 3D Model",
    extensions: &["bmd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x33, 0x44, 0x32, 0x62, 0x6D, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
