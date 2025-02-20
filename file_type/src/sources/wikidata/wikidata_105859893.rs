use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_893,
        source_type: SourceType::Wikidata,
        name: "Micro Fly Movie Format video",
        extensions: &["ufmf"],
        media_types: &["video/ufmf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x75, 0x66, 0x6D, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
