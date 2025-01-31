use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_82065829: FileFormat = FileFormat {
    id: 82_065_829,
    puid: "wikidata/82065829",
    name: "ChiWriter high resolution screen font",
    extensions: &["eft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
