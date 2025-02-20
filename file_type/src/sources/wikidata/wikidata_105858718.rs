use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858718: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_718,
        source_type: SourceType::Wikidata,
        name: "Palm GrayPaint bitmap",
        extensions: &["pdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x54, 0x41, 0x47, 0x72, 0x50, 0x3F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
