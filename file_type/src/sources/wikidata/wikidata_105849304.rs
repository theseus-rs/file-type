use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849304: FileFormat = FileFormat {
    id: 105_849_304,
    source_type: SourceType::Wikidata,
    name: "YouiDraw Drawing",
    extensions: &["ydr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x79, 0x64, 0x72, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
