use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849657: FileFormat = FileFormat {
    id: 105_849_657,
    puid: "wikidata/105849657",
    name: "Context Free design grammar",
    extensions: &["cfdg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x74, 0x61, 0x72, 0x74, 0x73, 0x68, 0x61, 0x70, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
