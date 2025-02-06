use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855624: FileFormat = FileFormat {
    id: 105_855_624,
    source_type: SourceType::Wikidata,
    name: "X-CAD Modifier Table",
    extensions: &["obj"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x4D, 0x54, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
