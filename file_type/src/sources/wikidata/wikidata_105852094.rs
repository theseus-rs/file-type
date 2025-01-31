use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852094: FileFormat = FileFormat {
    id: 105_852_094,
    puid: "wikidata/105852094",
    name: "Complete Statistica(l) System spreadsheet (v5)",
    extensions: &["sta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x53, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
