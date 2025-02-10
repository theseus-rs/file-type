use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_098,
        source_type: SourceType::Wikidata,
        name: "Adobe Photoshop Lightroom Template",
        extensions: &["lrtemplate"],
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
