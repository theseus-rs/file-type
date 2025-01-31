use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356290: FileFormat = FileFormat {
    id: 111_356_290,
    puid: "wikidata/111356290",
    name: "Yamaha Motif ES sample data file",
    extensions: &["w8v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
