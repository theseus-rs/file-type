use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127825832: FileFormat = FileFormat {
    id: 127_825_832,
    source_type: SourceType::Wikidata,
    name: "Cinema DTS Subtitle file format",
    extensions: &["sbt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCA, 0x00, 0x01, 0x00, 0x00, 0x00, 0x44, 0x54, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
