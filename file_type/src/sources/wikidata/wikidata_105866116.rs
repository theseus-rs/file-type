use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866116: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_116,
        source_type: SourceType::Wikidata,
        name: "KiCad Project (updated)",
        extensions: &["pro"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
