use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049372: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_372,
        source_type: SourceType::Wikidata,
        name: "ComputerEyes Raw Data Format, low resolution",
        extensions: &["ce1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x59, 0x45, 0x53, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
