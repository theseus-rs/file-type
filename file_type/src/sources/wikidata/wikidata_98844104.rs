use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_98844104: FileType = FileType {
    file_format: &FileFormat {
        id: 98_844_104,
        source_type: SourceType::Wikidata,
        name: "Grasshopper custom Layout",
        extensions: &["GHLAYOUT"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x46, 0x72, 0x61, 0x67, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x6E, 0x61,
                        0x6D, 0x65, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
