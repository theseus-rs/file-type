use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854916: FileFormat = FileFormat {
    id: 105_854_916,
    puid: "wikidata/105854916",
    name: "Compression Workshop compressed archive",
    extensions: &["cwf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x4A, 0x72])],
            },
        }],
    }],
    related_formats: &[],
};
