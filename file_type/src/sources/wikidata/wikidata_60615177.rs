use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60615177: FileFormat = FileFormat {
    id: 60_615_177,
    puid: "wikidata/60615177",
    name: "Serif DrawPlus Drawing, version 5",
    extensions: &["dpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
