use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857738: FileFormat = FileFormat {
    id: 105_857_738,
    puid: "wikidata/105857738",
    name: "86Box Floppy disk image (compressed)",
    extensions: &["86f"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x36, 0x62, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
