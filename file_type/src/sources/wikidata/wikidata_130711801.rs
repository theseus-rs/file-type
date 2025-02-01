use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130711801: FileFormat = FileFormat {
    id: 130_711_801,
    puid: "wikidata/130711801",
    name: "RPMSpec file format",
    extensions: &["spec"],
    media_types: &["text/x-rpm-spec"],
    internal_signatures: &[],
    related_formats: &[],
};
