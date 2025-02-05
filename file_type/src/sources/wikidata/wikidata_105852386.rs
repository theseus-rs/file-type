use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852386: FileFormat = FileFormat {
    id: 105_852_386,
    source_type: SourceType::Wikidata,
    name: "Wii Texture Animation",
    extensions: &["scn0"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x4E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
