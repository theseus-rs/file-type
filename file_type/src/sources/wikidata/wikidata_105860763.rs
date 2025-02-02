use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860763: FileFormat = FileFormat {
    id: 105_860_763,
    source_type: SourceType::Wikidata,
    name: "aSc Timetables timetable",
    extensions: &["roz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x05, 0x41, 0x53, 0x43, 0x54, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
