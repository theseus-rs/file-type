use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864896: FileFormat = FileFormat {
    id: 105_864_896,
    source_type: SourceType::Wikidata,
    name: "Palm iSilo 3.x document",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x6F, 0x63, 0x53, 0x69, 0x6C, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
