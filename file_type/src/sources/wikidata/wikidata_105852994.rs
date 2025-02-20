use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852994: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_994,
        source_type: SourceType::Wikidata,
        name: "SatcoDX channel list",
        extensions: &["sdx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
