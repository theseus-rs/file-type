use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65967080: FileFormat = FileFormat {
    id: 65_967_080,
    puid: "wikidata/65967080",
    name: "Sketch file format",
    extensions: &["sketch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
