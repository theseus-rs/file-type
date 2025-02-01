use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47493614: FileFormat = FileFormat {
    id: 47_493_614,
    puid: "wikidata/47493614",
    name: "Adobe InDesign Document, version CS3",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
