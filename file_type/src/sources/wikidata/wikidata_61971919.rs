use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61971919: FileFormat = FileFormat {
    id: 61_971_919,
    puid: "wikidata/61971919",
    name: "Microsoft Visual FoxPro Database Table File",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
