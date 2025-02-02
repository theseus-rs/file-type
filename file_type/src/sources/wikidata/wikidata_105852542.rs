use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852542: FileFormat = FileFormat {
    id: 105_852_542,
    source_type: SourceType::Wikidata,
    name: "StarOffice Drawing",
    extensions: &["sxd"],
    media_types: &["application/vnd.sun.xml.draw"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
