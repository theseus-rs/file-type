use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62414875: FileFormat = FileFormat {
    id: 62_414_875,
    puid: "wikidata/62414875",
    name: "XAML Binary Format",
    extensions: &["xbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
