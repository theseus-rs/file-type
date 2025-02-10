use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858552: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_552,
        source_type: SourceType::Wikidata,
        name: "ComputerEyes Raw Data Format bitmap (hi-res)",
        extensions: &["ce3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x59, 0x45, 0x53, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
