use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966886: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_886,
        source_type: SourceType::Wikidata,
        name: "Fuxoft AY Language",
        extensions: &["fxm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x58, 0x53, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
