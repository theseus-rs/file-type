use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867567: FileFormat = FileFormat {
    id: 105_867_567,
    puid: "wikidata/105867567",
    name: "Natron Node Preset",
    extensions: &["nps"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4E, 0x61, 0x74, 0x72, 0x6F, 0x6E, 0x20, 0x50, 0x72, 0x65, 0x73,
                    0x65, 0x74, 0x73, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
