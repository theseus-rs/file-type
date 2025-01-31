use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857297: FileFormat = FileFormat {
    id: 105_857_297,
    puid: "wikidata/105857297",
    name: "CZ Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x5A, 0x5F, 0x48, 0x45, 0x4C, 0x50, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
