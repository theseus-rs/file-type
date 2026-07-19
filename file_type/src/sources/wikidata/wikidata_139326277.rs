use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_139326277: FileType = FileType {
    file_format: &FileFormat {
        id: 139_326_277,
        source_type: SourceType::Wikidata,
        name: "Now Compress Archive",
        extensions: &["now"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x02, 0x00, 0x60])],
                },
            }],
        }],
        related_formats: &[],
    },
};
