use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131545359: FileFormat = FileFormat {
    id: 131_545_359,
    puid: "wikidata/131545359",
    name: "Cloud Optimized Point Cloud file",
    extensions: &["copc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
