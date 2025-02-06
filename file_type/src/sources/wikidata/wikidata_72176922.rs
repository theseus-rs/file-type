use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72176922: FileFormat = FileFormat {
    id: 72_176_922,
    source_type: SourceType::Wikidata,
    name: "KDevelop Project",
    extensions: &["KDEVPRJ"],
    media_types: &["text/ini"],
    signatures: &[],
    related_formats: &[],
};
