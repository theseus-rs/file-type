use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849804: FileFormat = FileFormat {
    id: 105_849_804,
    source_type: SourceType::Wikidata,
    name: "CWTool disk image (binary) (v3)",
    extensions: &["cwt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x77, 0x74, 0x6F, 0x6F, 0x6C, 0x20, 0x72, 0x61, 0x77, 0x20, 0x64, 0x61,
                    0x74, 0x61, 0x20, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
