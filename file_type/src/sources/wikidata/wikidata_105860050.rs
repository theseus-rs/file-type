use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860050: FileFormat = FileFormat {
    id: 105_860_050,
    source_type: SourceType::Wikidata,
    name: "MovieCD MVI1 Video",
    extensions: &["mvi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x04, 0x02, 0xDC])],
            },
        }],
    }],
    related_formats: &[],
};
