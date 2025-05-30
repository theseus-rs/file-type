use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_840,
        source_type: SourceType::Wikidata,
        name: "Genital Save state (v1.2+)",
        extensions: &["gns"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4E, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
