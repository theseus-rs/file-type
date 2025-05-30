use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856252: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_252,
        source_type: SourceType::Wikidata,
        name: "DXM music",
        extensions: &["dxm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
