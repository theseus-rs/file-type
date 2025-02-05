use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858865: FileFormat = FileFormat {
    id: 105_858_865,
    source_type: SourceType::Wikidata,
    name: "Biovision Hierarchy character animation format",
    extensions: &["bvh"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x49, 0x45, 0x52, 0x41, 0x52, 0x43, 0x48, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
