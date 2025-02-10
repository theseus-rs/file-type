use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856970: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_970,
        source_type: SourceType::Wikidata,
        name: "Ghemical Project",
        extensions: &["gpr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x67, 0x70, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
