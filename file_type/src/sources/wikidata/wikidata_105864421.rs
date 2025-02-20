use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864421: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_421,
        source_type: SourceType::Wikidata,
        name: "Papyrus document",
        extensions: &["pap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
