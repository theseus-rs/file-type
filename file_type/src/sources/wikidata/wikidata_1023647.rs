use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1023647: FileFormat = FileFormat {
    id: 1_023_647,
    source_type: SourceType::Wikidata,
    name: "Microsoft Compiled HTML Help",
    extensions: &["chm"],
    media_types: &["application/vnd.ms-htmlhelp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x54, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
