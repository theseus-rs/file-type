use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856290: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_290,
        source_type: SourceType::Wikidata,
        name: "Dream Station 1.0 module",
        extensions: &["dss"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x53, 0x46, 0x6D, 0x74, 0x31, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
