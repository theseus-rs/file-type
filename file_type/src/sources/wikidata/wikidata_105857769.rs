use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_769,
        source_type: SourceType::Wikidata,
        name: "Hercules FBA DASD image (compressed)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x42, 0x41, 0x5F, 0x43, 0x33, 0x37, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
