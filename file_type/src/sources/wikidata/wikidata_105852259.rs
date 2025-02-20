use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852259: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_259,
        source_type: SourceType::Wikidata,
        name: "PageRender3D Script",
        extensions: &["script"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
