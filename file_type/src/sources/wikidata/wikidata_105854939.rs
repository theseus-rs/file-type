use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854939: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_939,
        source_type: SourceType::Wikidata,
        name: "ENhanced Compressor compressed archive",
        extensions: &["enc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x6E, 0x63, 0x68, 0x00, 0x0F, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
