use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_067,
        source_type: SourceType::Wikidata,
        name: "SuperJAM! Patch",
        extensions: &["patch"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x55, 0x50, 0x52, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
