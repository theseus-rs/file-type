use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125523900: FileFormat = FileFormat {
    id: 125_523_900,
    puid: "wikidata/125523900",
    name: "Olympus RAW Detail Info file",
    extensions: &["ori"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
