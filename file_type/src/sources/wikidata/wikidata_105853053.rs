use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853053: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_053,
        source_type: SourceType::Wikidata,
        name: "Simis format (compressed)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x49, 0x4D, 0x49, 0x53, 0x41, 0x40, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
