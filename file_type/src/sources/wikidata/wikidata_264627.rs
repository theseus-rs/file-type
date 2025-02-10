use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_264627: FileType = FileType {
    file_format: &FileFormat {
        id: 264_627,
        source_type: SourceType::Wikidata,
        name: "Glulx",
        extensions: &["blb", "blorb", "gblorb", "glb", "ulx"],
        media_types: &["application/x-glulx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
