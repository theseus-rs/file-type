use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975617: FileFormat = FileFormat {
    id: 28_975_617,
    puid: "wikidata/28975617",
    name: "GNU Triangulated Surface",
    extensions: &["gts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
