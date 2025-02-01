use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119781139: FileFormat = FileFormat {
    id: 119_781_139,
    puid: "wikidata/119781139",
    name: "Street Atlas USA File",
    extensions: &["saf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
