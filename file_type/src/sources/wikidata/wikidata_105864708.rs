use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864708: FileFormat = FileFormat {
    id: 105_864_708,
    puid: "wikidata/105864708",
    name: "Extended TCPDUMP's style capture (little-endian)",
    extensions: &["pcap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0xCD, 0xB2, 0xA1])],
            },
        }],
    }],
    related_formats: &[],
};
