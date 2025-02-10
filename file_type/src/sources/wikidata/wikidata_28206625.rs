use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206625: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_625,
        source_type: SourceType::Wikidata,
        name: "MSX BASIC graphics",
        extensions: &["grp", "sc2", "sc5", "sc6", "sc7", "sc8", "scc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
