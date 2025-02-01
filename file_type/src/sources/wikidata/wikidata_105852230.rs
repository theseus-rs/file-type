use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852230: FileFormat = FileFormat {
    id: 105_852_230,
    puid: "wikidata/105852230",
    name: "Turbo Silver v3 Script",
    extensions: &["scr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x4C, 0x56, 0x45, 0x52, 0x20, 0x33, 0x0A, 0x53, 0x43, 0x52, 0x50,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
