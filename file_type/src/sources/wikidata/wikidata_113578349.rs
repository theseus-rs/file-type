use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113578349: FileFormat = FileFormat {
    id: 113_578_349,
    puid: "wikidata/113578349",
    name: "MAGIX Video File",
    extensions: &["mxv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x58, 0x52, 0x49, 0x46, 0x46, 0x36, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
