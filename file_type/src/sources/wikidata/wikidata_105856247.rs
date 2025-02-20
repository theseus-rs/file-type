use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856247: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_247,
        source_type: SourceType::Wikidata,
        name: "TomTom traffic data",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x41, 0x56, 0x54, 0x52, 0x41, 0x46, 0x46, 0x49, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
