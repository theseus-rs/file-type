use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867019: FileFormat = FileFormat {
    id: 105_867_019,
    source_type: SourceType::Wikidata,
    name: "Heath: The Unchosen Path game data archive",
    extensions: &["nrm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x65, 0x61, 0x74, 0x68, 0x3A, 0x20, 0x54, 0x68, 0x65, 0x20, 0x75, 0x6E,
                    0x63, 0x68, 0x6F, 0x73, 0x65, 0x6E, 0x20, 0x50, 0x61, 0x74, 0x68,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
