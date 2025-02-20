use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855280: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_280,
        source_type: SourceType::Wikidata,
        name: "GRX Font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x14, 0x02, 0x59, 0x19])],
                },
            }],
        }],
        related_formats: &[],
    },
};
