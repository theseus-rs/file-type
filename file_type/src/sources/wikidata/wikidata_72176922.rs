use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72176922: FileFormat = FileFormat {
    id: 72_176_922,
    puid: "wikidata/72176922",
    name: "KDevelop Project",
    extensions: &["KDEVPRJ"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
