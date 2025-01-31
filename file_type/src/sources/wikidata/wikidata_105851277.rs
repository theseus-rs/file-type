use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851277: FileFormat = FileFormat {
    id: 105_851_277,
    puid: "wikidata/105851277",
    name: "XM7 Tape image",
    extensions: &["t77"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x58, 0x4D, 0x37, 0x20, 0x54, 0x41, 0x50, 0x45, 0x20, 0x49, 0x4D, 0x41, 0x47,
                    0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
