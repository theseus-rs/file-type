use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762825: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_825,
        source_type: SourceType::Wikidata,
        name: "Darkroom Booth template",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x62, 0x64, 0x72])],
                },
            }],
        }],
        related_formats: &[],
    },
};
