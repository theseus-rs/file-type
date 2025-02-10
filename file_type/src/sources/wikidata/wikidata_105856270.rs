use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856270: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_270,
        source_type: SourceType::Wikidata,
        name: "Digital Sound Studio module",
        extensions: &["dss"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x55, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
