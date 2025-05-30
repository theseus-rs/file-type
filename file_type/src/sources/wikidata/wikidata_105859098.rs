use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_098,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 compressed/smooshed bitmap",
        extensions: &["spc", "sps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
