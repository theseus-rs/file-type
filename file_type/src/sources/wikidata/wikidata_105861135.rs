use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861135: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_135,
        source_type: SourceType::Wikidata,
        name: "RPG Maker 2000/2003 Map Tree",
        extensions: &["lmt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x4C, 0x63, 0x66, 0x4D, 0x61, 0x70, 0x54, 0x72, 0x65, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
