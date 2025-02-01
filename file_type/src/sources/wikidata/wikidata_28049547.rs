use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049547: FileFormat = FileFormat {
    id: 28_049_547,
    puid: "wikidata/28049547",
    name: "STAD image",
    extensions: &["pac", "seq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
