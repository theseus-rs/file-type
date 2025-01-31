use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858204: FileFormat = FileFormat {
    id: 105_858_204,
    puid: "wikidata/105858204",
    name: "EGrid32 Form properties template",
    extensions: &["egp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x47, 0x50, 0x52, 0x4F, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
