use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859794: FileFormat = FileFormat {
    id: 105_859_794,
    puid: "wikidata/105859794",
    name: "VisualBasic Project (Control)",
    extensions: &["vbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x79, 0x70, 0x65, 0x3D, 0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
