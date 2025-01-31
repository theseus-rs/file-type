use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112822096: FileFormat = FileFormat {
    id: 112_822_096,
    puid: "wikidata/112822096",
    name: "Strata StudioPro 3D File, version 1.75",
    extensions: &["vis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
