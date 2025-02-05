use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850135: FileFormat = FileFormat {
    id: 105_850_135,
    source_type: SourceType::Wikidata,
    name: "Flare3D Shader Language Compiled",
    extensions: &["compiled"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x53, 0x4C, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
