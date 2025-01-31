use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116619333: FileFormat = FileFormat {
    id: 116_619_333,
    puid: "wikidata/116619333",
    name: "Amiga SVX",
    extensions: &["svx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
