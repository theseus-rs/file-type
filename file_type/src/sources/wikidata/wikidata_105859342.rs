use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_342,
        source_type: SourceType::Wikidata,
        name: "QBX (MS Basic 7.x) Editor keyboard definition",
        extensions: &["key"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x43, 0x00, 0x02, 0x38, 0x3F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
