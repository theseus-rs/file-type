use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757998: FileFormat = FileFormat {
    id: 28_757_998,
    puid: "wikidata/28757998",
    name: "Inflate",
    extensions: &["infl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
