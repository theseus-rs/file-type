use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851224: FileFormat = FileFormat {
    id: 105_851_224,
    source_type: SourceType::Wikidata,
    name: "Abyss' Highest eXperience module (v2)",
    extensions: &["ahx"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x58, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
