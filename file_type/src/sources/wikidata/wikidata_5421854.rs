use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5421854: FileType = FileType {
    file_format: &FileFormat {
        id: 5_421_854,
        source_type: SourceType::Wikidata,
        name: "Extended Vector Animation",
        extensions: &["eva"],
        media_types: &["application/x-eva"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x0A, 0x45, 0x56, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
