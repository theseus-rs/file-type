use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851939: FileFormat = FileFormat {
    id: 105_851_939,
    source_type: SourceType::Wikidata,
    name: "GIMP Script-Fu Script",
    extensions: &["scm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
