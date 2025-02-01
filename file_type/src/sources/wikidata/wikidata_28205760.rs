use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205760: FileFormat = FileFormat {
    id: 28_205_760,
    puid: "wikidata/28205760",
    name: "Borland Graphics Interface image",
    extensions: &["bgi", "icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
