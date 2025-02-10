use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858059: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_059,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine graphic (v1)",
        extensions: &["mos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x53, 0x20, 0x56, 0x31, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
