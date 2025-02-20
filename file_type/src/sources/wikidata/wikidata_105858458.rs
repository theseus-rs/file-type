use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858458: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_458,
        source_type: SourceType::Wikidata,
        name: "Epanet data file",
        extensions: &["net"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x09, 0x3C, 0x45, 0x50, 0x41, 0x4E, 0x45, 0x54, 0x32, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
