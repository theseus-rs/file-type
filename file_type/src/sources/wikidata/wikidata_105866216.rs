use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866216: FileFormat = FileFormat {
    id: 105_866_216,
    source_type: SourceType::Wikidata,
    name: "Photodex ProShow Show file",
    extensions: &["psh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x68, 0x6F, 0x74, 0x6F, 0x64, 0x65, 0x78, 0x28, 0x52, 0x29, 0x20, 0x50,
                    0x72, 0x6F, 0x53, 0x68, 0x6F, 0x77, 0x28, 0x54, 0x4D, 0x29, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
