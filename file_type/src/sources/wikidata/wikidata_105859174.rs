use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859174: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_174,
        source_type: SourceType::Wikidata,
        name: "Streaming Progressive Image Format bitmap",
        extensions: &["spif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
