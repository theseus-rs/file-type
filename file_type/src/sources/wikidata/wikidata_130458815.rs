use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130458815: FileFormat = FileFormat {
    id: 130_458_815,
    puid: "wikidata/130458815",
    name: "ParaSail source code",
    extensions: &["psi"],
    media_types: &["text/x-parasail"],
    internal_signatures: &[],
    related_formats: &[],
};
