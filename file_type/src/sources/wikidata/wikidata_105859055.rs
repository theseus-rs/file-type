use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859055: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_055,
        source_type: SourceType::Wikidata,
        name: "BannerMania banner",
        extensions: &["bnr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD1, 0xBA, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
