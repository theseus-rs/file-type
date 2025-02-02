use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865490: FileFormat = FileFormat {
    id: 105_865_490,
    source_type: SourceType::Wikidata,
    name: "Programmer's Notepad Project",
    extensions: &["pnproj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x6E, 0x61, 0x6D, 0x65,
                    0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
