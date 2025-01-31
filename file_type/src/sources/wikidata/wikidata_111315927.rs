use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111315927: FileFormat = FileFormat {
    id: 111_315_927,
    puid: "wikidata/111315927",
    name: "Ensoniq EPS family instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
