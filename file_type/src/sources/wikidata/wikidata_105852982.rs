use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852982: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_982,
        source_type: SourceType::Wikidata,
        name: "STOS Var data",
        extensions: &["var"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x69, 0x6F, 0x6E, 0x70, 0x6F, 0x75, 0x76, 0x61, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
