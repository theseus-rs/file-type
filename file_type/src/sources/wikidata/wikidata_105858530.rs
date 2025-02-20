use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858530: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_530,
        source_type: SourceType::Wikidata,
        name: "ColoRIX bitmap",
        extensions: &["rix", "sci", "scx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
