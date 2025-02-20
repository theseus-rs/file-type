use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853817: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_817,
        source_type: SourceType::Wikidata,
        name: "UNIX Compressed data",
        extensions: &["z"],
        media_types: &["application/x-compress"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x9D, 0x90])],
                },
            }],
        }],
        related_formats: &[],
    },
};
