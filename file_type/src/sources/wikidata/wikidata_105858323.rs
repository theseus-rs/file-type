use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858323: FileFormat = FileFormat {
    id: 105_858_323,
    puid: "wikidata/105858323",
    name: "EasyPrint Preview",
    extensions: &["epp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x41, 0x53, 0x59, 0x50, 0x52, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x45, 0x56,
                    0x49, 0x45, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
