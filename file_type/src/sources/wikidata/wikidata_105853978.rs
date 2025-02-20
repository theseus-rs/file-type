use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853978: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_978,
        source_type: SourceType::Wikidata,
        name: "UltraCompressor 2 Encrypted archive",
        extensions: &["ue2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x45, 0x32, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
