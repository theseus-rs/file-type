use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852072: FileFormat = FileFormat {
    id: 105_852_072,
    puid: "wikidata/105852072",
    name: "Descent Game Save",
    extensions: &["sg0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x47, 0x53, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
