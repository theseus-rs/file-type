use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859296: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_296,
        source_type: SourceType::Wikidata,
        name: "Xerox EDMICS-RLC bitmap (var.1)",
        extensions: &["rlc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x61, 0x6E, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
