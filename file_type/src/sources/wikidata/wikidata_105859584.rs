use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859584: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_584,
        source_type: SourceType::Wikidata,
        name: "Visual Studio Visual C++ Project (UTF-8)",
        extensions: &["vcxproj"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
