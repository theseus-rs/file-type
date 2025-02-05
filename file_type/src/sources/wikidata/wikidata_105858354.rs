use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858354: FileFormat = FileFormat {
    id: 105_858_354,
    source_type: SourceType::Wikidata,
    name: "SilkRoad effect",
    extensions: &["efp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x4D, 0x58, 0x56, 0x45, 0x46, 0x46, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
