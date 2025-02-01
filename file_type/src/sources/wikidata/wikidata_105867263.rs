use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867263: FileFormat = FileFormat {
    id: 105_867_263,
    puid: "wikidata/105867263",
    name: "Terminate Smartnote",
    extensions: &["not"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4D, 0x41, 0x52, 0x54, 0x4E, 0x4F, 0x54, 0x45, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
