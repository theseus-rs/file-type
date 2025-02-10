use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_72199233: FileType = FileType {
    file_format: &FileFormat {
        id: 72_199_233,
        source_type: SourceType::Wikidata,
        name: "LilyPond music score",
        extensions: &["ily", "ly"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
