use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860752: FileFormat = FileFormat {
    id: 105_860_752,
    puid: "wikidata/105860752",
    name: "RenderWare 3d model",
    extensions: &["rwx"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x42, 0x65, 0x67, 0x69, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
