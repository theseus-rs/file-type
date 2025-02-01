use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127266247: FileFormat = FileFormat {
    id: 127_266_247,
    puid: "wikidata/127266247",
    name: "Assembly file",
    extensions: &["eaf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
