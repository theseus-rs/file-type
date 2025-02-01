use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855798: FileFormat = FileFormat {
    id: 105_855_798,
    puid: "wikidata/105855798",
    name: "Delphi Options File",
    extensions: &["dof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
