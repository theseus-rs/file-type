use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_13454995: FileFormat = FileFormat {
    id: 13_454_995,
    puid: "wikidata/13454995",
    name: "DVD data file and backup data file",
    extensions: &["bup", "ifo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
