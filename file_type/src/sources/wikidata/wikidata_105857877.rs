use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857877: FileFormat = FileFormat {
    id: 105_857_877,
    puid: "wikidata/105857877",
    name: "PS2 Memory Card image",
    extensions: &["ps2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x6F, 0x6E, 0x79, 0x20, 0x50, 0x53, 0x32, 0x20, 0x4D, 0x65, 0x6D, 0x6F,
                    0x72, 0x79, 0x20, 0x43, 0x61, 0x72, 0x64, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61,
                    0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
