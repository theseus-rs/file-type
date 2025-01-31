use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859032: FileFormat = FileFormat {
    id: 105_859_032,
    puid: "wikidata/105859032",
    name: "Binary Point File 3",
    extensions: &["bpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x50, 0x46, 0x21, 0x30, 0x30, 0x30, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
