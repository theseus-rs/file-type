use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851939: FileFormat = FileFormat {
    id: 105_851_939,
    source_type: SourceType::Wikidata,
    name: "GIMP Script-Fu Script",
    extensions: &["scm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
