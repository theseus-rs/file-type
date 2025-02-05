use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855598: FileFormat = FileFormat {
    id: 105_855_598,
    source_type: SourceType::Wikidata,
    name: "Oberon/F Symbol File",
    extensions: &["osf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x4F, 0x6F])],
            },
        }],
    }],
    related_formats: &[],
};
