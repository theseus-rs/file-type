use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661259: FileFormat = FileFormat {
    id: 112_661_259,
    puid: "wikidata/112661259",
    name: "LightWave LScript File",
    extensions: &["ls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
