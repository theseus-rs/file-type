use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858313: FileFormat = FileFormat {
    id: 105_858_313,
    puid: "wikidata/105858313",
    name: "Etherpad document",
    extensions: &["etherpad"],
    media_types: &["text/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x22, 0x70, 0x61, 0x64, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
