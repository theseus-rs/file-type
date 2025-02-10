use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_323,
        source_type: SourceType::Wikidata,
        name: "Adobe Photoshop Lightroom template",
        extensions: &["agtemplate"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x20, 0x3D, 0x20, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
