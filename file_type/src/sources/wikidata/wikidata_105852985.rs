use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852985: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_985,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Sound",
        extensions: &["snd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x54, 0x45, 0x56, 0x45, 0x02, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
