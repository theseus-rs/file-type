use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15838827: FileType = FileType {
    file_format: &FileFormat {
        id: 15_838_827,
        source_type: SourceType::Wikidata,
        name: "TopoJSON",
        extensions: &["topojson"],
        media_types: &["application/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3A, 0x22, 0x54, 0x6F, 0x70,
                        0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x22, 0x2C, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
