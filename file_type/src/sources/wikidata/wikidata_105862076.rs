use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_076,
        source_type: SourceType::Wikidata,
        name: "mzTab-M format",
        extensions: &["mztab"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
