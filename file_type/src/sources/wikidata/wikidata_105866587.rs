use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866587: FileFormat = FileFormat {
    id: 105_866_587,
    puid: "wikidata/105866587",
    name: "Clarion Project",
    extensions: &["prj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x2D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
