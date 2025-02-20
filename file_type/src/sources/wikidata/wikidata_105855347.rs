use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855347: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_347,
        source_type: SourceType::Wikidata,
        name: "form-Z compiled script",
        extensions: &["fsb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x5A, 0x53, 0x43, 0x00, 0x00, 0x00, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
