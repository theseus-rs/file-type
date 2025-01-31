use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975875: FileFormat = FileFormat {
    id: 28_975_875,
    puid: "wikidata/28975875",
    name: "OOGL TLIST Projective file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
