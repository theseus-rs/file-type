use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866986: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_986,
        source_type: SourceType::Wikidata,
        name: "Mathematica Notebook (headerless)",
        extensions: &["nb"],
        media_types: &["application/vnd.wolfram.mathematica"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x6F, 0x74, 0x65, 0x62, 0x6F, 0x6F, 0x6B, 0x5B, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
