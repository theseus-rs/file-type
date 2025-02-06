use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856153: FileFormat = FileFormat {
    id: 105_856_153,
    source_type: SourceType::Wikidata,
    name: "Macintosh Disk image (BZlib compressed)",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    signatures: &[],
    related_formats: &[],
};
