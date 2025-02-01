use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113376365: FileFormat = FileFormat {
    id: 113_376_365,
    puid: "wikidata/113376365",
    name: "Yamaha Wave Audio Generic",
    extensions: &["f01", "s01", "u01", "w01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
