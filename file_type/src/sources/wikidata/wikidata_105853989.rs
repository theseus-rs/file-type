use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853989: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_989,
        source_type: SourceType::Wikidata,
        name: "Squeeze It compressed archive",
        extensions: &["sqz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4C, 0x53, 0x51, 0x5A, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
