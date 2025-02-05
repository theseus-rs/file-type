use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862893: FileFormat = FileFormat {
    id: 105_862_893,
    source_type: SourceType::Wikidata,
    name: "Mutation Annotation Format",
    extensions: &["maf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x75, 0x67, 0x6F, 0x5F, 0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
