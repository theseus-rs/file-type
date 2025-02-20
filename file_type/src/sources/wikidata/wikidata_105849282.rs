use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849282: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_282,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto 5 Texture Dictionary",
        extensions: &["ytd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x53, 0x43, 0x37, 0x0D, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
