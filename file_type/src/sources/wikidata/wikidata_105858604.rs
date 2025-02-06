use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858604: FileFormat = FileFormat {
    id: 105_858_604,
    source_type: SourceType::Wikidata,
    name: "CHP! - CH compressor for Picture! compressed bitmap",
    extensions: &["chp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x50, 0x21, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
