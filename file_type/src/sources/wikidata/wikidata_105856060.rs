use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_060,
        source_type: SourceType::Wikidata,
        name: "Descent 2 alternative texture set",
        extensions: &["pog"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x50, 0x4F, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
