use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_596391: FileType = FileType {
    file_format: &FileFormat {
        id: 596_391,
        source_type: SourceType::Wikidata,
        name: "APK",
        extensions: &["apk", "apkm", "apks", "xapk"],
        media_types: &["application/vnd.android.package-archive"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
