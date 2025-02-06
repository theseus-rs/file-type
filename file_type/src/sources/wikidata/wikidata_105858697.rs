use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858697: FileFormat = FileFormat {
    id: 105_858_697,
    source_type: SourceType::Wikidata,
    name: "Cryo Interactive gamedata",
    extensions: &["bf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x72, 0x79, 0x6F, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
