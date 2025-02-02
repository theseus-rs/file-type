use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72176922: FileFormat = FileFormat {
    id: 72_176_922,
    source_type: SourceType::Wikidata,
    name: "KDevelop Project",
    extensions: &["KDEVPRJ"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
