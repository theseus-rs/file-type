use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854269: FileFormat = FileFormat {
    id: 105_854_269,
    source_type: SourceType::Wikidata,
    name: "Advanced Module Format",
    extensions: &["amf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
