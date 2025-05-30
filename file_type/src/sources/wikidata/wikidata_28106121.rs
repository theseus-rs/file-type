use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28106121: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_121,
        source_type: SourceType::Wikidata,
        name: "PXM",
        extensions: &["pxm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x58, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
