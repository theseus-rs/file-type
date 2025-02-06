use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854826: FileFormat = FileFormat {
    id: 105_854_826,
    source_type: SourceType::Wikidata,
    name: "AIMP Skin (v4)",
    extensions: &["acs4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
