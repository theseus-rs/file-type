use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853774: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_774,
        source_type: SourceType::Wikidata,
        name: "qpress compressed archive",
        extensions: &["qp"],
        media_types: &["application/x-qpress"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x71, 0x70, 0x72, 0x65, 0x73, 0x73, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
