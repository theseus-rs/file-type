use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118583205: FileFormat = FileFormat {
    id: 118_583_205,
    puid: "wikidata/118583205",
    name: "Project5 Project",
    extensions: &["p5p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
