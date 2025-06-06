use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861404: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_404,
        source_type: SourceType::Wikidata,
        name: "BSD library",
        extensions: &["a"],
        media_types: &["application/x-archive"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
