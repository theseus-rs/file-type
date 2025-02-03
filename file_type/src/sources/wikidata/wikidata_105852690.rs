use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852690: FileFormat = FileFormat {
    id: 105_852_690,
    source_type: SourceType::Wikidata,
    name: "InfoPal Structure",
    extensions: &["str"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x0A, 0x0A, 0x0A, 0x0D, 0x49, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                    0x69, 0x6F, 0x6E, 0x20, 0x50, 0x61, 0x6C, 0x61, 0x63, 0x65, 0x20, 0x28, 0x74,
                    0x6D, 0x29, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x49, 0x6E, 0x66, 0x6F,
                    0x50, 0x61, 0x6C, 0x20, 0x28, 0x74, 0x6D, 0x29, 0x20, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
