use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852282: FileFormat = FileFormat {
    id: 105_852_282,
    puid: "wikidata/105852282",
    name: "SUNTronic module",
    extensions: &["sun"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0xE7, 0xFF, 0xFE, 0x4D, 0xFA])],
            },
        }],
    }],
    related_formats: &[],
};
