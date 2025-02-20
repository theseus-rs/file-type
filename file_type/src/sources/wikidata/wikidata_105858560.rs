use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858560: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_560,
        source_type: SourceType::Wikidata,
        name: "PrintFox/Pagefox bitmap (640x400)",
        extensions: &["bg", "bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
