use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50604441: FileType = FileType {
    file_format: &FileFormat {
        id: 50_604_441,
        source_type: SourceType::Wikidata,
        name: "SNAP Archive Data File",
        extensions: &["adf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x75, 0x72, 0x76, 0x65, 0x79])],
                },
            }],
        }],
        related_formats: &[],
    },
};
