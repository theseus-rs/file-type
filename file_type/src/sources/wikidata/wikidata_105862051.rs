use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862051: FileFormat = FileFormat {
    id: 105_862_051,
    puid: "wikidata/105862051",
    name: "PTC Creo Material (with BOM)",
    extensions: &["mtl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x4E, 0x44, 0x5F, 0x52, 0x65, 0x6C, 0x50, 0x61, 0x72, 0x53,
                    0x65, 0x74, 0x5F, 0x4B, 0x30, 0x31, 0x20, 0x3D, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
