use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859163: FileFormat = FileFormat {
    id: 105_859_163,
    source_type: SourceType::Wikidata,
    name: "Xerox EDMICS-RLC bitmap (var.2)",
    extensions: &["rlc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4C, 0x43, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
