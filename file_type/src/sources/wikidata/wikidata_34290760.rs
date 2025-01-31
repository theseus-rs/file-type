use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34290760: FileFormat = FileFormat {
    id: 34_290_760,
    puid: "wikidata/34290760",
    name: "Statistical Package for the Social Sciences syntax file",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
