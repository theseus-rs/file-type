use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858536: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_536,
        source_type: SourceType::Wikidata,
        name: "EPOC/Symbian exported MultiBitMap",
        extensions: &["mbm"],
        media_types: &["image/x-epoc-mbm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x8A, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
