use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852077: FileFormat = FileFormat {
    id: 105_852_077,
    puid: "wikidata/105852077",
    name: "Sandcastle Help File Builder project",
    extensions: &["shfb"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x73,
                    0x63, 0x68, 0x65, 0x6D, 0x61, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
