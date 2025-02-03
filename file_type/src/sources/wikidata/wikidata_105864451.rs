use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864451: FileFormat = FileFormat {
    id: 105_864_451,
    source_type: SourceType::Wikidata,
    name: "CMN Phonebook",
    extensions: &["phb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x4D, 0x4E, 0x20, 0x50, 0x48, 0x4F, 0x4E, 0x45, 0x42, 0x4F, 0x4F,
                    0x4B, 0x20, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
