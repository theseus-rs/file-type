use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851929: FileFormat = FileFormat {
    id: 105_851_929,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Workspace",
    extensions: &["sublime-workspace"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x0A, 0x09, 0x22, 0x61, 0x75, 0x74, 0x6F, 0x5F, 0x63, 0x6F, 0x6D, 0x70,
                    0x6C, 0x65, 0x74, 0x65, 0x22, 0x3A, 0x0A, 0x09, 0x7B, 0x0A, 0x09, 0x09, 0x22,
                    0x73, 0x65, 0x6C, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5F, 0x69, 0x74, 0x65, 0x6D,
                    0x73, 0x22, 0x3A, 0x0A, 0x09, 0x09, 0x5B, 0x0A, 0x09, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
