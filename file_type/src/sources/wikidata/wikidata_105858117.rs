use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_117,
        source_type: SourceType::Wikidata,
        name: "VTrucco disk image",
        extensions: &["vtr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x54, 0x72, 0x75, 0x63, 0x63, 0x6F, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
