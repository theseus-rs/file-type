use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344723: FileFormat = FileFormat {
    id: 28_344_723,
    puid: "wikidata/28344723",
    name: "Turbo Pascal chain file",
    extensions: &["chn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
