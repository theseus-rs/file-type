use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109017314: FileFormat = FileFormat {
    id: 109_017_314,
    puid: "wikidata/109017314",
    name: "iZotope RX document",
    extensions: &["rxdoc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x22, 0xC8, 0xC9, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
