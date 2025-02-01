use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049603: FileFormat = FileFormat {
    id: 28_049_603,
    puid: "wikidata/28049603",
    name: "Tiny Stuff, medium resolution",
    extensions: &["tn2", "tny"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
