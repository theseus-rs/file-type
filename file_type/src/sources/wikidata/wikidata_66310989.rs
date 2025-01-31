use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66310989: FileFormat = FileFormat {
    id: 66_310_989,
    puid: "wikidata/66310989",
    name: "Pantry Files",
    extensions: &["pl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
