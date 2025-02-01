use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852832: FileFormat = FileFormat {
    id: 105_852_832,
    puid: "wikidata/105852832",
    name: "SharkPort file",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x00, 0x00, 0x00, 0x53, 0x68, 0x61, 0x72, 0x6B, 0x50, 0x6F, 0x72, 0x74,
                    0x53, 0x61, 0x76, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
