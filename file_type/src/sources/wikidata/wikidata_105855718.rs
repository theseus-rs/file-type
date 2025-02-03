use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855718: FileFormat = FileFormat {
    id: 105_855_718,
    source_type: SourceType::Wikidata,
    name: "RollerCoaster Tycoon 3 game data",
    extensions: &["ovl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x47, 0x52, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
