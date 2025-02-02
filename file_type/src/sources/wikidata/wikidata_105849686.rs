use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849686: FileFormat = FileFormat {
    id: 105_849_686,
    source_type: SourceType::Wikidata,
    name: "CHAOSultdGEM parameters",
    extensions: &["chs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x53, 0x75, 0x6C, 0x74, 0x64, 0x35, 0x54, 0x4D, 0x4D, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
