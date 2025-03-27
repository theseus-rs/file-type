use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111843814: FileType = FileType {
    file_format: &FileFormat {
        id: 111_843_814,
        source_type: SourceType::Wikidata,
        name: "Quite OK Image Format",
        extensions: &["qoi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x71, 0x6F, 0x69, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
