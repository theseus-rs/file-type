use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861993: FileFormat = FileFormat {
    id: 105_861_993,
    puid: "wikidata/105861993",
    name: "Wireshark Micropross mplog",
    extensions: &["mplog"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x43, 0x53, 0x49, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
