use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859938: FileFormat = FileFormat {
    id: 105_859_938,
    source_type: SourceType::Wikidata,
    name: "Weston CAPture video (BE)",
    extensions: &["wcap"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x43, 0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
