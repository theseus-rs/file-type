use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113376732: FileFormat = FileFormat {
    id: 113_376_732,
    puid: "wikidata/113376732",
    name: "Easy CD Creator Layout, version 5-6",
    extensions: &["cl5", "rcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
