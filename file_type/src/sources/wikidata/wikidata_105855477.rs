use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855477: FileFormat = FileFormat {
    id: 105_855_477,
    puid: "wikidata/105855477",
    name: "PLS-CADD Feature code",
    extensions: &["fea"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x59, 0x50, 0x45, 0x3D, 0x27, 0x46, 0x45, 0x41, 0x20, 0x46, 0x49, 0x4C,
                    0x45, 0x27, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
