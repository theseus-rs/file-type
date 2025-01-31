use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857737: FileFormat = FileFormat {
    id: 105_857_737,
    puid: "wikidata/105857737",
    name: "Colour Genie high level tape image",
    extensions: &["cgc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6C, 0x6F, 0x75, 0x72, 0x20, 0x47, 0x65, 0x6E, 0x69, 0x65, 0x20,
                    0x2D, 0x20, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x54, 0x61, 0x70,
                    0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
