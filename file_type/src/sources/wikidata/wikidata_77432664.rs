use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77432664: FileFormat = FileFormat {
    id: 77_432_664,
    puid: "wikidata/77432664",
    name: "InfoPath Template Part",
    extensions: &["xtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
