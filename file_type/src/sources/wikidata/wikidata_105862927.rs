use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862927: FileFormat = FileFormat {
    id: 105_862_927,
    source_type: SourceType::Wikidata,
    name: "ztracker module",
    extensions: &["zt"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x54, 0x68, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
