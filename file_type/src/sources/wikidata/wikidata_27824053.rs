use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824053: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_053,
        source_type: SourceType::Wikidata,
        name: "ar, AIX small archive format variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x61, 0x69, 0x61, 0x66, 0x66, 0x3E, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
