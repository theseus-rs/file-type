use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858072: FileFormat = FileFormat {
    id: 105_858_072,
    source_type: SourceType::Wikidata,
    name: "T98-Next harddisk image",
    extensions: &["nhd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x39, 0x38, 0x48, 0x44, 0x44, 0x49, 0x4D, 0x41, 0x47, 0x45, 0x2E, 0x52,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
