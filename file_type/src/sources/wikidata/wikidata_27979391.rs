use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979391: FileFormat = FileFormat {
    id: 27_979_391,
    puid: "wikidata/27979391",
    name: "ANSI Animation",
    extensions: &["ans", "vt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
