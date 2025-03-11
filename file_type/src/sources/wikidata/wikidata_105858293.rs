use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858293: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_293,
        source_type: SourceType::Wikidata,
        name: "EPOC Data database",
        extensions: &[],
        media_types: &["application/x-epoc-data"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x86, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
