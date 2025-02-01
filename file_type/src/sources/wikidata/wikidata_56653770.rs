use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56653770: FileFormat = FileFormat {
    id: 56_653_770,
    puid: "wikidata/56653770",
    name: "Final Cut Pro Project",
    extensions: &["fcp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA2, 0x4B, 0x65, 0x79, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
