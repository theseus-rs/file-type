use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129188124: FileFormat = FileFormat {
    id: 129_188_124,
    puid: "wikidata/129188124",
    name: "FreeFem++ source code file",
    extensions: &["edp"],
    media_types: &["text/x-freefem"],
    internal_signatures: &[],
    related_formats: &[],
};
