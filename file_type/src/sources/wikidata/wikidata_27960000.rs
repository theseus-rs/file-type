use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960000: FileFormat = FileFormat {
    id: 27_960_000,
    puid: "wikidata/27960000",
    name: "Perfect Clarity Audio",
    extensions: &["pca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
