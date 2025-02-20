use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205452: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_452,
        source_type: SourceType::Wikidata,
        name: "J6I",
        extensions: &["j6i"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x3E, 0x44, 0x53, 0x43, 0x49, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
