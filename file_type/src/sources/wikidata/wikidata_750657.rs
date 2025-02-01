use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_750657: FileFormat = FileFormat {
    id: 750_657,
    puid: "wikidata/750657",
    name: "Quicken Interchange Format",
    extensions: &["qif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x54, 0x79, 0x70, 0x65, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
