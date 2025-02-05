use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967110: FileFormat = FileFormat {
    id: 27_967_110,
    source_type: SourceType::Wikidata,
    name: "Abyss' Highest eXperience",
    extensions: &["ahx"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x58, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
