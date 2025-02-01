use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823998: FileFormat = FileFormat {
    id: 27_823_998,
    puid: "wikidata/27823998",
    name: "Maptech Update Patch File, version 3.0",
    extensions: &["ptc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
