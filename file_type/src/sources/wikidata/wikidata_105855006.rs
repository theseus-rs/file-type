use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855006: FileFormat = FileFormat {
    id: 105_855_006,
    source_type: SourceType::Wikidata,
    name: "Proline Voice Data",
    extensions: &["pvd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x12, 0x50, 0x72, 0x6F, 0x6C, 0x69, 0x6E, 0x65, 0x20, 0x56, 0x6F, 0x69, 0x63,
                    0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
