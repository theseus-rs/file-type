use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859558: FileFormat = FileFormat {
    id: 105_859_558,
    source_type: SourceType::Wikidata,
    name: "Time Shift Video",
    extensions: &["tsv"],
    media_types: &["video/x-tsv"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x53, 0x52, 0x46, 0x32, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
