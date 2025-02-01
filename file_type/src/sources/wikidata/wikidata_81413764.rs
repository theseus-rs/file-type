use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81413764: FileFormat = FileFormat {
    id: 81_413_764,
    puid: "wikidata/81413764",
    name: "EnCase Case data",
    extensions: &["cas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5F, 0x43, 0x41, 0x53, 0x45, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
