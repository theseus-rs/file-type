use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206135: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_135,
        source_type: SourceType::Wikidata,
        name: "Fractint FRA",
        extensions: &["fra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x37, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
