use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049610: FileFormat = FileFormat {
    id: 28_049_610,
    puid: "wikidata/28049610",
    name: "Tiny Stuff, high resolution",
    extensions: &["tn3", "tny"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
