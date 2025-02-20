use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_245,
        source_type: SourceType::Wikidata,
        name: "Tecplot Layout Package",
        extensions: &["lpk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x50, 0x4B, 0x38, 0x30, 0x30, 0x30, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
