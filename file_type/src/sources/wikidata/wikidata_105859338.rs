use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859338: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_338,
        source_type: SourceType::Wikidata,
        name: "Quartus Block Design File",
        extensions: &["bdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
