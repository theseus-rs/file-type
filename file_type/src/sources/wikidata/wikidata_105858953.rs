use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858953: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_953,
        source_type: SourceType::Wikidata,
        name: "bigBed Track Format",
        extensions: &["bb", "bigbed"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEB, 0xF2, 0x89, 0x87, 0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
