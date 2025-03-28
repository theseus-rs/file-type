use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207085: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_085,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Graphics file",
        extensions: &["shp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0B, 0x34, 0x58, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
