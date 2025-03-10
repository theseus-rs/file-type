use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_060,
        source_type: SourceType::Wikidata,
        name: "X-CAD Screen Menu",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
