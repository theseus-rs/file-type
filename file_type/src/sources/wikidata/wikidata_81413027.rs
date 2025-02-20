use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81413027: FileType = FileType {
    file_format: &FileFormat {
        id: 81_413_027,
        source_type: SourceType::Wikidata,
        name: "Capella sheet data file",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x61, 0x70, 0x33, 0x2D, 0x76, 0x3A, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
