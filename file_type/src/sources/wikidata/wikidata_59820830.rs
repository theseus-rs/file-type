use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59820830: FileFormat = FileFormat {
    id: 59_820_830,
    puid: "wikidata/59820830",
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
