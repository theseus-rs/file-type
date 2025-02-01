use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48911845: FileFormat = FileFormat {
    id: 48_911_845,
    puid: "wikidata/48911845",
    name: "Hewlett Packard AdvanceWrite Text File",
    extensions: &["aw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
