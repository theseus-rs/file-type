use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852759: FileFormat = FileFormat {
    id: 105_852_759,
    puid: "wikidata/105852759",
    name: "DeLorme vector Symbol data",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x4C, 0x6F, 0x72, 0x6D, 0x65, 0x20, 0x56, 0x65, 0x63, 0x74, 0x6F,
                    0x72, 0x20, 0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x20, 0x44, 0x61, 0x74, 0x61,
                    0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
