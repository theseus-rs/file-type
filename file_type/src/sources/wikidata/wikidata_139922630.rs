use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_139922630: FileType = FileType {
    file_format: &FileFormat {
        id: 139_922_630,
        source_type: SourceType::Wikidata,
        name: "VideoClix Project File",
        extensions: &["vcx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x6B, 0x4C, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
