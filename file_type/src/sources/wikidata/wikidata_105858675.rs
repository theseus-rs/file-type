use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858675: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_675,
        source_type: SourceType::Wikidata,
        name: "J Wavelet Image Codec bitmap",
        extensions: &["wic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFA, 0xDE, 0xBA, 0xBE, 0x01, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
