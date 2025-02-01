use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863439: FileFormat = FileFormat {
    id: 105_863_439,
    puid: "wikidata/105863439",
    name: "Picasa movie project",
    extensions: &["mxf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x54, 0x69, 0x6D, 0x65, 0x6C, 0x69,
                    0x6E, 0x65, 0x3E, 0x0D, 0x0A, 0x20, 0x3C, 0x63, 0x75, 0x72, 0x72, 0x65, 0x73,
                    0x6F, 0x6C, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
