use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205879: FileFormat = FileFormat {
    id: 28_205_879,
    puid: "wikidata/28205879",
    name: "CUT",
    extensions: &["cut"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
