use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858439: FileFormat = FileFormat {
    id: 105_858_439,
    source_type: SourceType::Wikidata,
    name: "easyHDR 3 Project",
    extensions: &["ehpx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x65, 0x61, 0x73, 0x79, 0x48, 0x44, 0x52, 0x20, 0x33, 0x20, 0x70, 0x72, 0x6F,
                    0x6A, 0x65, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
