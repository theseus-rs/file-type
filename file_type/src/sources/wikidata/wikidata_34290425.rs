use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34290425: FileFormat = FileFormat {
    id: 34_290_425,
    puid: "wikidata/34290425",
    name: "Statistical Package for the Social Sciences output file",
    extensions: &["spo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
