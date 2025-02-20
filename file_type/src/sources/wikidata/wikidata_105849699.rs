use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_699,
        source_type: SourceType::Wikidata,
        name: "Clonk Material definition",
        extensions: &["c4m"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
