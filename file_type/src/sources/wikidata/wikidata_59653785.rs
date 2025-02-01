use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59653785: FileFormat = FileFormat {
    id: 59_653_785,
    puid: "wikidata/59653785",
    name: "Maya Binary File Format, 32 bit",
    extensions: &["mb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
