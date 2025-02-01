use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862566: FileFormat = FileFormat {
    id: 105_862_566,
    puid: "wikidata/105862566",
    name: "Windows Manifest - Visual Stylesheet XML file",
    extensions: &["manifest"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
